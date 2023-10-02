use petgraph::{prelude::DiGraphMap, algo::TarjanScc};
use rustc_type_ir::TyKind::{Adt, self};
use utils::{compiler_interface::Program, data_structure::vec_vec::VecVec};
use std::mem::MaybeUninit;

/// All possible access paths w.r.t. pointers within a fixed depth of `K_LIMIT`.
/// Paths are annotated with range of numbers that indicate the preorder travesals
pub struct AccessPaths<const K_LIMIT: usize> {
    /// A map of type: `depth -> struct_id -> field_idx -> offset`.
    /// The last entries of inner vecs store the sizes of the stuct_id
    offsets: [VecVec<usize>; K_LIMIT],
}

impl<const K_LIMIT: usize> AccessPaths<K_LIMIT> {
    pub fn new(program: &Program) -> Self {
        let &Program { tcx, ref structs, .. } = program;
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

        todo!()

        // let mut post_order: Vec<DefId> = Vec::with_capacity(structs.len());
        // TarjanScc::new().run(&graph, |nodes| post_order.extend(nodes));

        // let mut offsets = vec![];

        // // The base case
        // let mut offset = VecVec::with_indices_capacity(post_order.len());
        // for &did in &post_order {
        //     let ty = tcx.type_of(did).skip_binder();
        //     let Adt(adt_def, _) = ty.kind() else {
        //         unreachable!("impossible")
        //     };
        //     assert!(adt_def.is_struct());
        //     offset.push_element(0);
        //     for _ in adt_def.all_fields() {
        //         offset.push_element(0);
        //     }
        //     offset.complete_cur_vec();
        // }
        // offsets.push(offset.complete());


        // AccessPaths { offsets: offsets.try_into().unwrap() }
    }
}

pub struct PathMatcher {
    // steps: Vec<impl Iterator<Item = Ty>>,
}

// /// Pick the `depth` subtree shaped by `ty` and `chased`
// pub fn subtree(ty: Ty, chased: usize, depth: usize) -> impl Iterator<Item = usize> {
//     // The depth of the current tree is `K_LIMIT - chased`
//     if depth + chased == K_LIMIT {
//         return tree(ty, chased);
//     }
// }
