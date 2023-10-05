use std::ops::Range;

use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_abi::FieldIdx;
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
    struct_idx: FxHashMap<DefId, StructIdx>,
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
        let struct_idx: FxHashMap<DefId, StructIdx> = post_order
            .iter_enumerated()
            .map(|(idx, &did)| (did, idx))
            .collect();

        let offsets = Offsets::new(&post_order, tcx);

        AccessPaths {
            post_order,
            struct_idx,
            offsets,
            leaves: todo!(),
        }
    }

    /// Number of pointers reachable from `ty`
    pub fn size_of(&self, depth: usize, ty: Ty<'tcx>) -> usize {
        assert!(depth <= K_LIMIT);
        let (levels, maybe_adt) = trim_pointers(ty);
        if levels >= depth {
            return depth;
        } else if let Some(&struct_idx) = maybe_adt.and_then(|adt| self.struct_idx.get(&adt.did()))
        {
            return levels + self.offsets.size_of(depth - levels, struct_idx);
        }
        return levels;
    }
}

/// A map of type: `depth -> struct_id -> IndexVec<field_idx, offset>`.
pub struct Offsets<const K_LIMIT: usize> {
    one_level_size: usize,
    /// struct_idx -> offset_start..offset_end
    structs_indices: IndexVec<StructIdx, usize>,
    offsets: Vec<usize>,
}

impl<const K_LIMIT: usize> Offsets<K_LIMIT> {
    fn offset_of(&self, depth: usize, struct_idx: StructIdx, field_idx: FieldIdx) -> usize {
        let start = self.structs_indices[struct_idx];
        self.offsets[self.one_level_size * depth + start + field_idx.as_usize()]
    }

    fn offsets_of(&self, depth: usize, struct_idx: StructIdx) -> &[usize] {
        let start = self.structs_indices[struct_idx];
        let end = self.structs_indices[struct_idx + 1];
        &self.offsets[self.one_level_size * depth + start..self.one_level_size * depth + end]
    }

    fn size_of(&self, depth: usize, struct_idx: StructIdx) -> usize {
        let end = self.structs_indices[struct_idx + 1];
        self.offsets[self.one_level_size * depth + end]
    }

    fn new(post_order: &IndexVec<StructIdx, DefId>, tcx: TyCtxt) -> Self {
        let mut structs_indices = IndexVec::with_capacity(post_order.len() + 1);
        let mut offsets = Vec::new();

        let mut one_level_size = 0;
        let mut struct_start_index = 0;

        for def_id in post_order.iter() {
            structs_indices.push(struct_start_index);

            let ty = tcx.type_of(def_id).skip_binder();
            let Adt(adt_def, _) = ty.kind() else {
                unreachable!("impossible")
            };
            assert!(adt_def.is_struct());

            for _ in adt_def.all_fields() {
                offsets.push(0);
                struct_start_index += 1;
                one_level_size += 1;
            }
        }
        structs_indices.push(struct_start_index);

        assert_eq!(offsets.len(), one_level_size);

        let mut offsets = Offsets {
            one_level_size,
            structs_indices,
            offsets,
        };

        offsets.construct(post_order, tcx);

        offsets
    }

    fn construct(&mut self, post_order: &IndexVec<StructIdx, DefId>, tcx: TyCtxt) {
        for depth in 1..K_LIMIT + 1 {
            for (struct_idx, def_id) in post_order.iter_enumerated() {
                let Adt(adt_def, subst_ref) = tcx.type_of(def_id).skip_binder().kind() else {
                    unreachable!("impossible")
                };
                assert!(adt_def.is_struct());

                let mut offset = 0;
                for field_def in adt_def.all_fields() {
                    self.offsets.push(offset);
                    let field_ty = field_def.ty(tcx, subst_ref);
                    let (levels, maybe_adt) = trim_pointers(field_ty);
                    if levels >= depth {
                        offset += depth;
                    } else {
                        todo!()
                    }
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
