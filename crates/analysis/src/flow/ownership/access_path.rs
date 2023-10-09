use either::Either::{Left, Right};
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{HasLocalDecls, Place, ProjectionElem},
    ty::{AdtDef, Ty, TyCtxt},
};
use rustc_span::def_id::DefId;
use rustc_type_ir::TyKind::{self, Adt};
use utils::compiler_interface::Program;

utils::macros::newtype_index! {
    #[debug_format = "{}"]
    struct StructIdx {
    }
}

/// All possible access paths w.r.t. pointers within a fixed depth of `K_LIMIT`.
/// Paths are annotated with range of numbers that indicate the preorder travesals
pub struct AccessPaths<const K_LIMIT: usize> {
    post_order: IndexVec<StructIdx, DefId>,
    indices: FxHashMap<DefId, StructIdx>,
    offsets: Offsets<K_LIMIT>,
    leaves: Leaves<K_LIMIT>,
}

pub type Path<T> = (T, Projections);

/// `(offset_relative_to_base, num_pointers_reachable, depth)`
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Projections(pub(super) usize, pub(super) usize, pub(super) usize);

impl Projections {
    pub const fn offset(self) -> usize {
        self.0
    }

    pub const fn size(self) -> usize {
        self.1
    }

    pub const fn depth(self) -> usize {
        self.2
    }
}

impl std::fmt::Display for Projections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Path (offset: {}, size: {}, depth: {}",
            self.0, self.1, self.2
        ))
    }
}

impl<const K_LIMIT: usize> AccessPaths<K_LIMIT> {
    pub fn new(program: &Program) -> Self {
        let &Program {
            tcx, ref structs, ..
        } = program;
        let mut graph = DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(*did);
        });
        for did in structs.iter() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).skip_binder().kind() else {
                unreachable!("impossible")
            };
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let mut ty = field_def.ty(tcx, subst_ref);
                while let TyKind::Array(inner_ty, _) = ty.kind() {
                    ty = *inner_ty;
                }
                if let TyKind::Adt(sub_adt_def, _) = ty.kind() {
                    if graph.contains_node(sub_adt_def.did()) {
                        graph.add_edge(*did, sub_adt_def.did(), ());
                    }
                }
            }
        }

        let mut post_order: IndexVec<StructIdx, DefId> = IndexVec::with_capacity(structs.len());
        TarjanScc::new().run(&graph, |nodes| post_order.extend(nodes.iter().copied()));
        let indices: FxHashMap<DefId, StructIdx> = post_order
            .iter_enumerated()
            .map(|(idx, &did)| (did, idx))
            .collect();

        let offsets = Offsets::new(&post_order, &indices, tcx);
        let leaves = Leaves::new(&post_order, &indices, tcx, &offsets);

        AccessPaths {
            post_order,
            indices,
            offsets,
            leaves,
        }
    }

    pub fn post_order(&self) -> &[DefId] {
        &self.post_order.raw
    }

    fn offsets_of(&self, depth: usize, did: DefId) -> &[usize] {
        assert!(depth <= K_LIMIT);
        self.offsets.offsets_of(depth, self.indices[&did])
    }

    fn size_of_inner(
        &self,
        depth: usize,
        num_wrapping_pointers: usize,
        maybe_struct_idx: Option<StructIdx>,
    ) -> usize {
        if num_wrapping_pointers >= depth {
            return depth;
        } else if let Some(struct_idx) = maybe_struct_idx {
            return num_wrapping_pointers
                + self
                    .offsets
                    .size_of(depth - num_wrapping_pointers, struct_idx);
        }
        return num_wrapping_pointers;
    }

    /// Number of pointers reachable from `ty`
    pub fn size_of(&self, depth: usize, ty: Ty) -> usize {
        assert!(depth <= K_LIMIT);
        let (num_wrapping_pointers, maybe_adt) = unwrap_pointers(ty);
        self.size_of_inner(
            depth,
            num_wrapping_pointers,
            maybe_adt.and_then(|adt_def| self.indices.get(&adt_def.did()).copied()),
        )
    }

    /// Get the `(offset, size, depth)` of an access_path
    ///
    /// TODO replace `HasLocalDecls` with `HasPathCache`?
    pub fn projections<'tcx, D: HasLocalDecls<'tcx>>(
        &self,
        place: &Place<'tcx>,
        local_decls: &D,
    ) -> Projections {
        let mut offset = 0;
        let mut ty = local_decls.local_decls()[place.local].ty;
        let mut levels = 0;
        for proj_elem in place.projection {
            if levels == K_LIMIT {
                return Projections(offset, 0, 0);
            }
            match proj_elem {
                ProjectionElem::Deref => {
                    offset += 1;
                    ty = ty.builtin_deref(true).unwrap().ty;
                    levels += 1;
                }
                ProjectionElem::Field(field_idx, field_ty) => {
                    let Adt(adt_def, _) = ty.kind() else {
                        unreachable!()
                    };
                    // TODO unions???
                    if adt_def.is_union() {
                        return Projections(offset, 0, 0);
                    }
                    assert!(adt_def.is_struct());
                    offset +=
                        self.offsets_of(K_LIMIT - levels, adt_def.did())[field_idx.as_usize()];
                    ty = field_ty;
                }
                ProjectionElem::Index(_) => ty = ty.builtin_index().unwrap(),
                ProjectionElem::ConstantIndex { .. }
                | ProjectionElem::Subslice { .. }
                | ProjectionElem::Downcast(_, _)
                | ProjectionElem::OpaqueCast(_) => unreachable!(),
            }
        }
        Projections(offset, self.size_of(K_LIMIT - levels, ty), K_LIMIT - levels)
    }

    /// Patch up offsets of a type `ty` used with `depth` in a context `depth + delta`.
    ///
    /// `delta == 0` => `patch_up(depth, delta, ty) = 0..size_of(depth, ty)`
    pub fn patch_up(&self, depth: usize, delta: usize, ty: Ty) -> impl Iterator<Item = usize> + '_ {
        assert!(depth + delta <= K_LIMIT);
        let (levels, maybe_adt) = unwrap_pointers(ty);
        if levels >= depth {
            Left(0..depth)
        } else if let Some(&struct_idx) = maybe_adt.and_then(|adt| self.indices.get(&adt.did())) {
            let mut leaves = self
                .leaves
                .leaves_of(depth, struct_idx)
                .into_iter()
                .map(
                    move |&(num_wrapping_pointers, maybe_struct_idx, position)| {
                        (
                            position,
                            self.size_of_inner(delta, num_wrapping_pointers, maybe_struct_idx),
                        )
                    },
                )
                .peekable();

            Right(
                (0..self.size_of_inner(depth, levels, Some(struct_idx)))
                    .enumerate()
                    .scan(0, move |state, (index, offset)| {
                        let position = offset + *state;
                        if let Some(&(position, size)) = leaves.peek() {
                            if position == index {
                                let _ = leaves.next();
                                *state += size;
                            }
                        }
                        Some(position)
                    }),
            )
        } else {
            Left(0..levels)
        }
    }
}

/// A map of type: `depth -> struct_id -> IndexVec<field_idx, offset>`.
pub struct Offsets<const K_LIMIT: usize> {
    size_per_depth: usize,
    /// struct_idx -> offset_start..offset_end
    structs_indices: IndexVec<StructIdx, usize>,
    offsets: Vec<usize>,
}

impl<const K_LIMIT: usize> Offsets<K_LIMIT> {
    fn offsets_of(&self, depth: usize, struct_idx: StructIdx) -> &[usize] {
        let start = self.structs_indices[struct_idx];
        let end = self.structs_indices[struct_idx + 1];
        &self.offsets[self.size_per_depth * depth + start..self.size_per_depth * depth + end]
    }

    fn size_of(&self, depth: usize, struct_idx: StructIdx) -> usize {
        let end = self.structs_indices[struct_idx + 1] - 1;
        self.offsets[self.size_per_depth * depth + end]
    }

    fn new(
        post_order: &IndexVec<StructIdx, DefId>,
        indices: &FxHashMap<DefId, StructIdx>,
        tcx: TyCtxt,
    ) -> Self {
        let mut structs_indices = IndexVec::with_capacity(post_order.len() + 1);
        let mut offsets = Vec::new();

        structs_indices.push(offsets.len());
        for def_id in post_order.iter() {
            let ty = tcx.type_of(def_id).skip_binder();
            let Adt(adt_def, _) = ty.kind() else {
                unreachable!("impossible")
            };
            assert!(adt_def.is_struct());

            offsets.push(0);
            for _ in adt_def.all_fields() {
                offsets.push(0);
            }

            structs_indices.push(offsets.len());
        }
        let size_per_depth = offsets.len();

        let mut offsets = Offsets {
            size_per_depth,
            structs_indices,
            offsets,
        };

        offsets.construct(post_order, indices, tcx);

        offsets
    }

    fn construct(
        &mut self,
        post_order: &IndexVec<StructIdx, DefId>,
        indices: &FxHashMap<DefId, StructIdx>,
        tcx: TyCtxt,
    ) {
        for depth in 1..K_LIMIT + 1 {
            for def_id in post_order.iter() {
                let Adt(adt_def, subst_ref) = tcx.type_of(def_id).skip_binder().kind() else {
                    unreachable!("impossible")
                };
                assert!(adt_def.is_struct());

                let mut offset = 0;
                self.offsets.push(offset);
                for field_def in adt_def.all_fields() {
                    let field_ty = field_def.ty(tcx, subst_ref);
                    let (levels, maybe_adt) = unwrap_pointers(field_ty);
                    if levels >= depth {
                        offset += depth;
                    } else if let Some(&child_idx) =
                        maybe_adt.and_then(|adt| indices.get(&adt.did()))
                    {
                        offset += levels + self.size_of(depth - levels, child_idx);
                    } else {
                        offset += levels;
                    }
                    self.offsets.push(offset);
                }
            }
        }
    }
}

/// A map of type: `depth -> struct_id -> Vec<(ty, position)>`
/// that records the type and position information of a leaf node in
/// an access path
pub struct Leaves<const K_LIMIT: usize> {
    number_per_depth: usize,
    structs_indices: Vec<usize>,
    /// (num_wrapping_pointers, option_struct_idx, position)
    leaves: Vec<(usize, Option<StructIdx>, usize)>,
}

impl<const K_LIMIT: usize> Leaves<K_LIMIT> {
    fn leaves_of(
        &self,
        depth: usize,
        struct_idx: StructIdx,
    ) -> &[(usize, Option<StructIdx>, usize)] {
        &self.leaves[self.structs_indices[struct_idx.as_usize() + self.number_per_depth * depth]
            ..self.structs_indices[struct_idx.as_usize() + self.number_per_depth * depth + 1]]
    }

    fn new(
        post_order: &IndexVec<StructIdx, DefId>,
        indices: &FxHashMap<DefId, StructIdx>,
        tcx: TyCtxt,
        offsets: &Offsets<K_LIMIT>,
    ) -> Self {
        let mut structs_indices = Vec::with_capacity(post_order.len() + 1);
        let leaves = vec![];

        structs_indices.push(leaves.len());
        for _ in post_order {
            structs_indices.push(leaves.len());
        }

        let mut leaves = Self {
            number_per_depth: post_order.len() + 1,
            structs_indices,
            leaves,
        };
        leaves.construct(post_order, indices, tcx, offsets);
        leaves
    }

    fn construct(
        &mut self,
        post_order: &IndexVec<StructIdx, DefId>,
        indices: &FxHashMap<DefId, StructIdx>,
        tcx: TyCtxt,
        offsets: &Offsets<K_LIMIT>,
    ) {
        let mut buffer = vec![];
        for depth in 1..K_LIMIT + 1 {
            self.structs_indices.push(self.leaves.len());
            for (struct_idx, def_id) in post_order.iter_enumerated() {
                // compute leaves
                let ty = tcx.type_of(def_id).skip_binder();
                let Adt(adt_def, subst_ref) = ty.kind() else {
                    unreachable!("impossible")
                };
                assert!(adt_def.is_struct());
                for (field_def, &offset) in adt_def
                    .all_fields()
                    .zip(offsets.offsets_of(depth, struct_idx))
                {
                    let field_ty = field_def.ty(tcx, subst_ref);
                    let (levels, maybe_adt) = unwrap_pointers(field_ty);
                    let maybe_struct_idx =
                        maybe_adt.and_then(|adt_def| indices.get(&adt_def.did()).copied());

                    // what if no pointers? optimisation?
                    if levels >= depth {
                        self.leaves.push((levels - depth, maybe_struct_idx, offset))
                    } else if let Some(child_idx) = maybe_struct_idx {
                        buffer.extend_from_slice(self.leaves_of(depth - levels, child_idx));
                        for (num_wrapping_pointers, maybe_struct_idx, position) in buffer.drain(0..)
                        {
                            self.leaves.push((
                                num_wrapping_pointers,
                                maybe_struct_idx,
                                offset + levels + position,
                            ));
                        }
                    }
                }

                self.structs_indices.push(self.leaves.len());
            }
        }
    }
}

/// Decompose a type into levels of outside pointers and a (possible) adt
fn unwrap_pointers(ty: Ty) -> (usize, Option<AdtDef>) {
    let mut ty = ty;

    let mut level_pointers = 0;
    loop {
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }

        if let Some(ty_mut) = ty.builtin_deref(true) {
            ty = ty_mut.ty;
            level_pointers += 1;
            continue;
        }

        break;
    }

    (level_pointers, ty.ty_adt_def())
}

#[cfg(test)]
mod tests {
    use rustc_middle::ty::Adt;

    use super::AccessPaths;
    use crate::flow::ownership::access_path::Projections;

    #[test]
    fn test_rng_structs() {
        const PROGRAM: &str = "struct s {
    f: t,
    g: *mut i32,
    h: *mut s,
}
struct t {
    f: *mut i32,
    g: u
}
struct u {
    f: v,
    g: w,
    h: x
}
struct v {
    f: i32,
}
struct w {
    f: *mut i32
}
struct x;";
        utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
            macro_rules! define_structs {
                ($( $x: ident ),*) => {
                    $(
                        let &$x = program
                            .structs
                            .iter()
                            .find(|&&did| {
                            let stringify!($x) = program.tcx.def_path_str(did).as_str() else { return false };
                            true
                        })
                        .unwrap();
                    )*
                };
            }
            define_structs!(s, t, u, v, w, x);

            const K_LIMIT: usize = 3;
            let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);

            assert_eq!(access_paths.offsets_of(0, s), [0, 0, 0, 0]);
            assert_eq!(access_paths.offsets_of(1, s), [0, 2, 3, 4]);
            assert_eq!(access_paths.offsets_of(2, s), [0, 2, 3, 8]);
            assert_eq!(access_paths.offsets_of(3, s), [0, 2, 3, 12]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, t), [0, 1, 2]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, u), [0, 0, 1, 1]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, v), [0, 0]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, w), [0, 1]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, x), [0]);

            for depth in 0..K_LIMIT + 1 {
                for s in [s, t, u, v, w, x].map(|s| program.tcx.type_of(s).skip_binder()) {
                    assert!(access_paths
                        .patch_up(depth, 0, s)
                        .eq(0..access_paths.size_of(depth, s)))
                }
            }
        })
    }

    #[test]
    fn bst_offsets() {
        const PROGRAM: &str =
            "struct Node { data: Data, left: *mut Node, right: *mut Node } struct Data;";
        utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
            const K_LIMIT: usize = 3;
            let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);

            let &node = access_paths
                .post_order
                .iter()
                .find(|&&did| {
                    let "Node" = program.tcx.def_path_str(did).as_str() else {
                        return false;
                    };
                    true
                })
                .unwrap();

            assert_eq!(access_paths.offsets_of(0, node), [0, 0, 0, 0]);
            assert_eq!(access_paths.offsets_of(1, node), [0, 0, 1, 2]);
            assert_eq!(access_paths.offsets_of(2, node), [0, 0, 3, 6]);
            assert_eq!(access_paths.offsets_of(3, node), [0, 0, 7, 14]);
        })
    }

    #[test]
    fn bst_leaves() {
        const PROGRAM: &str = "struct Node { left: *mut Node, right: *mut Node }";
        utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
            const K_LIMIT: usize = 4;
            let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);

            let &node_def_id = access_paths
                .post_order
                .iter()
                .find(|&&did| {
                    let "Node" = program.tcx.def_path_str(did).as_str() else {
                        return false;
                    };
                    true
                })
                .unwrap();

            assert_eq!(access_paths.offsets_of(0, node_def_id), [0, 0, 0]);
            assert_eq!(access_paths.offsets_of(1, node_def_id), [0, 1, 2]);
            assert_eq!(access_paths.offsets_of(2, node_def_id), [0, 3, 6]);
            assert_eq!(access_paths.offsets_of(3, node_def_id), [0, 7, 14]);

            let node = program.tcx.type_of(node_def_id).skip_binder();
            let Adt(adt_def, subst_ref) = node.kind() else {
                unreachable!()
            };
            let node_pointer = adt_def
                .all_fields()
                .next()
                .unwrap()
                .ty(program.tcx, &subst_ref);

            assert!(access_paths.patch_up(1, 2, node_pointer).eq([0]));
            assert!(access_paths.patch_up(2, 1, node_pointer).eq([0, 1, 4]));
            assert!(access_paths.patch_up(2, 2, node_pointer).eq([0, 1, 8]));
            assert!(access_paths
                .patch_up(3, 1, node_pointer)
                .eq([0, 1, 2, 5, 8, 9, 12]));
            for depth in 0..K_LIMIT + 1 {
                assert!(&access_paths
                    .patch_up(depth, 0, node)
                    .eq(0..access_paths.size_of(depth, node)));
                assert!(&access_paths
                    .patch_up(depth, 0, node_pointer)
                    .eq(0..access_paths.size_of(depth, node_pointer)));
            }
        })
    }

    #[test]
    fn bst_paths() {
        const PROGRAM: &str = "struct Node { left: *mut Node, right: *mut Node }
fn f(input: *mut Node) {
    (*input).left = core::ptr::null_mut();
}";
        utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
            const K_LIMIT: usize = 3;
            let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);

            let &node_def_id = access_paths
                .post_order
                .iter()
                .find(|&&did| {
                    let "Node" = program.tcx.def_path_str(did).as_str() else {
                        return false;
                    };
                    true
                })
                .unwrap();
            let node = program.tcx.type_of(node_def_id).skip_binder();
            let Adt(adt_def, subst_ref) = node.kind() else {
                unreachable!()
            };
            let node_pointer = adt_def
                .all_fields()
                .next()
                .unwrap()
                .ty(program.tcx, &subst_ref);

            use rustc_abi::FieldIdx;
            use rustc_middle::mir::{Local, Place, ProjectionElem};

            let body = program.tcx.optimized_mir(program.fns.first().unwrap());

            // input
            assert_eq!(
                access_paths.projections(&Place::from(Local::from_u32(1)), body),
                Projections(0, 7, 3)
            );

            // *input
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1))
                        .project_deeper(&[ProjectionElem::Deref], program.tcx),
                    body
                ),
                Projections(1, 6, 2)
            );

            // (*input).left
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(0), node_pointer),
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(1, 3, 2)
            );

            // *(*input).left
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(0), node_pointer),
                            ProjectionElem::Deref,
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(2, 2, 1)
            );

            // (*(*input).left).right
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(0), node_pointer),
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(1), node_pointer),
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(3, 1, 1)
            );

            // (*input).right
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(1), node_pointer),
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(4, 3, 2)
            );

            // *(*input).right
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(1), node_pointer),
                            ProjectionElem::Deref,
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(5, 2, 1)
            );

            // (*(*input).right).left
            assert_eq!(
                access_paths.projections(
                    &Place::from(Local::from_u32(1)).project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(1), node_pointer),
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(0), node_pointer),
                        ],
                        program.tcx
                    ),
                    body
                ),
                Projections(5, 1, 1)
            );
        })
    }
}
