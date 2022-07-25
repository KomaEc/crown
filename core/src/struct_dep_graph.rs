use graph_ext::implementation::sparse_bit_vector;
use rustc_data_structures::graph::iterate::post_order_from;
use rustc_hir::def_id::DefId;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::ty::{TyCtxt, TyKind};

/// This structure encodes direct dependencies between
/// user defined structs.
/// For instance, in `struct S { f: T } struct T;`
/// `S` is considered dependent on `T`;
/// in `struct S { f: *mut T } struct T;`
/// `S` is not considered dependent on `T`
pub struct StructDepGraph {
    /// Sorted by `DefId` to support fast inverse lookup
    pub structs: IndexVec<Struct, DefId>,
    pub post_order: Vec<DefId>,
    pub graph: sparse_bit_vector::SparseBitVectorGraph<Struct>,
}

impl StructDepGraph {
    pub fn new(tcx: TyCtxt, mut structs: Vec<DefId>) -> Self {
        structs.sort();
        let structs = IndexVec::from_raw(structs);
        let graph = sparse_bit_vector::SparseBitVectorGraph::new(
            structs.len(),
            structs.iter_enumerated().flat_map(|(sid, &did)| {
                let adt_def = tcx.adt_def(did);
                assert!(adt_def.is_struct());
                let structs_ref = &structs;
                adt_def.all_fields().filter_map(move |field_def| {
                    let ty = tcx.type_of(field_def.did);
                    if let TyKind::Adt(sub_adt_def, _) = ty.kind() {
                        return structs_ref
                            .binary_search(&sub_adt_def.did())
                            .map(|sub_sid| (sid, sub_sid))
                            .ok();
                    }
                    None
                })
            }),
        );

        let mut visited = BitSet::new_empty(structs.len());
        let mut post_order = Vec::with_capacity(structs.len());
        for sid in structs.indices() {
            if !visited.contains(sid) {

                // struct dep graph must be acyclic
                debug_assert!(
                    rustc_data_structures::graph::iterate::TriColorDepthFirstSearch::new(&graph)
                        .run_from(sid, &mut rustc_data_structures::graph::iterate::CycleDetector)
                        .is_none()
                );

                let mut nodes = post_order_from(&graph, sid)
                    .into_iter()
                    .map(|sid| {
                        assert!(visited.insert(sid));
                        todo!("debug");
                        structs[sid]
                    })
                    .collect();
                post_order.append(&mut nodes);
            }
        }

        assert_eq!(post_order.len(), structs.len());

        StructDepGraph {
            structs,
            post_order,
            graph,
        }
    }
}

crate::macros::newtype_index! {
    pub struct Struct {
        DEBUG_FORMAT = "Struct_({})"
    }
}

#[cfg(test)]
mod tests {
    use crate::test::run_compiler_with;
    use rustc_data_structures::graph::{WithNumEdges, WithNumNodes};

    #[test]
    fn test1() {
        const PROGRAM: &str = "
        struct S {
            f: T,
        }
        struct T;
        ";

        run_compiler_with(PROGRAM.into(), |program| {
            let struct_dep = &program.struct_dep_graph;
            let s = struct_dep.structs[0u32.into()];
            let t = struct_dep.structs[1u32.into()];
            // let s = tcx.adt_def(s);
            // let t = tcx.adt_def(t);
            assert_eq!(program.tcx.def_path_str(s), "S");
            assert_eq!(program.tcx.def_path_str(t), "T");

            assert_eq!(struct_dep.graph.num_nodes(), 2);
            assert_eq!(struct_dep.graph.num_edges(), 1);

            // S -> T already exists
            assert!(struct_dep.graph.has_edge(0u32.into(), 1u32.into()));

            // post_order
            assert_eq!(&struct_dep.post_order, &[t, s])
        })
    }

    #[test]
    fn test2() {
        const PROGRAM: &str = "
        struct S {
            f: *mut T,
            g: Box<T>,
            h: &'static T,
        }
        struct T;
        ";

        run_compiler_with(PROGRAM.into(), |program| {
            let struct_dep = program.struct_dep_graph;
            let s = struct_dep.structs[0u32.into()];
            let t = struct_dep.structs[1u32.into()];
            // let s = tcx.adt_def(s);
            // let t = tcx.adt_def(t);
            assert_eq!(program.tcx.def_path_str(s), "S");
            assert_eq!(program.tcx.def_path_str(t), "T");

            assert_eq!(struct_dep.graph.num_nodes(), 2);

            // no dependency
            assert_eq!(struct_dep.graph.num_edges(), 0);
        })
    }
}
