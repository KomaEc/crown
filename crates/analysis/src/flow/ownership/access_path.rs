use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_index::IndexVec;
use rustc_middle::ty::{AdtDef, Ty, TyCtxt};
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
pub struct AccessPaths<'tcx, const K_LIMIT: usize> {
    post_order: IndexVec<StructIdx, DefId>,
    indices: FxHashMap<DefId, StructIdx>,
    offsets: Offsets<K_LIMIT>,
    leaves: Leaves<'tcx, K_LIMIT>,
}

impl<'tcx, const K_LIMIT: usize> AccessPaths<'tcx, K_LIMIT> {
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
        let leaves = Leaves::new();

        AccessPaths {
            post_order,
            indices,
            offsets,
            leaves,
        }
    }

    pub fn offsets_of(&self, depth: usize, did: DefId) -> &[usize] {
        assert!(depth <= K_LIMIT);
        self.offsets.offsets_of(depth, self.indices[&did])
    }

    /// Number of pointers reachable from `ty`
    pub fn size_of(&self, depth: usize, ty: Ty<'tcx>) -> usize {
        assert!(depth <= K_LIMIT);
        let (levels, maybe_adt) = trim_pointers(ty);
        if levels >= depth {
            return depth;
        } else if let Some(&struct_idx) = maybe_adt.and_then(|adt| self.indices.get(&adt.did())) {
            return levels + self.offsets.size_of(depth - levels, struct_idx);
        }
        return levels;
    }
}

/// A map of type: `depth -> struct_id -> IndexVec<field_idx, offset>`.
pub struct Offsets<const K_LIMIT: usize> {
    level_size: usize,
    /// struct_idx -> offset_start..offset_end
    structs_indices: IndexVec<StructIdx, usize>,
    offsets: Vec<usize>,
}

impl<const K_LIMIT: usize> Offsets<K_LIMIT> {
    // fn offset_of(&self, depth: usize, struct_idx: StructIdx, field_idx: FieldIdx) -> usize {
    //     let start = self.structs_indices[struct_idx];
    //     self.offsets[self.one_level_size * depth + start + field_idx.as_usize()]
    // }

    fn offsets_of(&self, depth: usize, struct_idx: StructIdx) -> &[usize] {
        let start = self.structs_indices[struct_idx];
        let end = self.structs_indices[struct_idx + 1];
        &self.offsets[self.level_size * depth + start..self.level_size * depth + end]
    }

    fn size_of(&self, depth: usize, struct_idx: StructIdx) -> usize {
        let end = self.structs_indices[struct_idx + 1] - 1;
        self.offsets[self.level_size * depth + end]
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
        let level_size = offsets.len();

        let mut offsets = Offsets {
            level_size,
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
                    let (levels, maybe_adt) = trim_pointers(field_ty);
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
pub struct Leaves<'tcx, const K_LIMIT: usize> {
    depths_indices: Vec<usize>,
    structs_indices: IndexVec<StructIdx, usize>,
    leaves: Vec<(Ty<'tcx>, u32)>,
}

impl<'tcx, const K_LIMIT: usize> Leaves<'tcx, K_LIMIT> {
    fn new() -> Self {
        Self {
            depths_indices: vec![],
            structs_indices: IndexVec::new(),
            leaves: vec![],
        }
    }
}

// /// Pick the `depth` subtree shaped by `ty` and `chased`
// pub fn subtree(ty: Ty, chased: usize, depth: usize) -> impl Iterator<Item = usize> {
//     // The depth of the current tree is `K_LIMIT - chased`
//     if depth + chased == K_LIMIT {
//         return tree(ty, chased);
//     }
// }

/// Decompose a type into levels of outside pointers and a (possible) adt
fn trim_pointers(ty: Ty) -> (usize, Option<AdtDef>) {
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
    use super::AccessPaths;

    #[test]
    fn test_0() {
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
            let access_paths: AccessPaths<'_, K_LIMIT> = AccessPaths::new(&program);

            assert_eq!(access_paths.offsets_of(0, s), [0, 0, 0, 0]);
            assert_eq!(access_paths.offsets_of(1, s), [0, 2, 3, 4]);
            assert_eq!(access_paths.offsets_of(2, s), [0, 2, 3, 8]);
            assert_eq!(access_paths.offsets_of(3, s), [0, 2, 3, 12]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, t), [0, 1, 2]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, u), [0, 0, 1, 1]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, v), [0, 0]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, w), [0, 1]);
            assert_eq!(access_paths.offsets_of(K_LIMIT, x), [0]);
        })
    }

    #[test]
    fn test2() {
        const PROGRAM: &str =
            "struct Node { data: Data, left: *mut Node, right: *mut Node } struct Data;";
        utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
            const K_LIMIT: usize = 3;
            let access_paths: AccessPaths<'_, K_LIMIT> = AccessPaths::new(&program);

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
}
