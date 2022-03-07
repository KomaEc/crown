use graph::{delegate_graph_via, implementation::forward_star};
use rustc_data_structures::graph::{
    DirectedGraph, GraphPredecessors, GraphSuccessors, WithNumEdges, WithNumNodes,
    WithPredecessors, WithSuccessors,
};
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Location, Terminator, TerminatorKind},
    ty::{TyCtxt, TyKind::FnDef},
};

pub struct CallGraph {
    /// Invariant: the set of functions are sorted by `DefId` to facilitate
    /// reverse lookup
    pub functions: IndexVec<Func, DefId>,
    pub call_sites: IndexVec<CallSite, Location>,
    pub graph: forward_star::Graph<Func, CallSite>,
}

delegate_graph_via!(CallGraph.graph: forward_star::Graph<Func, CallSite>);

impl CallGraph {
    pub fn new(tcx: TyCtxt, bodies: impl Iterator<Item = DefId>) -> Self {
        let mut bodies = bodies.collect::<Vec<_>>();
        bodies.sort();
        let num_nodes = bodies.len();
        CallGraphConstruction {
            tcx,
            functions: IndexVec::from_raw(bodies),
            call_sites: IndexVec::new(),
            call_graph: forward_star::Graph::new(num_nodes, std::iter::empty()),
        }
        .construct()
    }
}

impl CallGraph {
    #[inline]
    pub fn lookup_function(&self, did: &DefId) -> Result<Func, Func> {
        self.functions.binary_search(did)
    }
}

rustc_index::newtype_index! {
    pub struct Func {
        DEBUG_FORMAT = "Function_({})"
    }
}

rustc_index::newtype_index! {
    pub struct CallSite {
        DEBUG_FORMAT = "CallSite_({})"
    }
}

struct CallGraphConstruction<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub functions: IndexVec<Func, DefId>,
    pub call_sites: IndexVec<CallSite, Location>,
    pub call_graph: forward_star::Graph<Func, CallSite>,
}

impl<'tcx> CallGraphConstruction<'tcx> {
    pub fn construct(mut self) -> CallGraph {
        for &did in &self.functions {
            let body = self.tcx.optimized_mir(did);
            CallGraphNodeVis {
                tcx: self.tcx,
                this: self.functions.binary_search(&did).unwrap(),
                functions: &self.functions,
                call_sites: &mut self.call_sites,
                call_graph: &mut self.call_graph,
            }
            .visit_body(body);
        }
        CallGraph {
            functions: self.functions,
            call_sites: self.call_sites,
            graph: self.call_graph,
        }
    }
}

struct CallGraphNodeVis<'me, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub this: Func,
    pub functions: &'me IndexVec<Func, DefId>,
    pub call_sites: &'me mut IndexVec<CallSite, Location>,
    pub call_graph: &'me mut forward_star::Graph<Func, CallSite>,
}

impl<'me, 'tcx> Visitor<'tcx> for CallGraphNodeVis<'me, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator, location: Location) {
        if let TerminatorKind::Call {
            func,
            args: _,
            destination: _,
            cleanup: _,
            from_hir_call: _,
            fn_span: _,
        } = &terminator.kind
        {
            let ty = func
                .constant()
                .expect("closures or function pointers are not supported!")
                .ty();
            if let &FnDef(callee_did, _generic_args) = ty.kind() {
                // local defined functions: libc externs or user functions
                if let Some(did) = callee_did.as_local() {
                    // if it is user functions
                    if matches!(
                        self.tcx.hir().find_by_def_id(did),
                        Some(rustc_hir::Node::Item(_))
                    ) {
                        let other = self.functions.binary_search(&callee_did).unwrap();
                        assert_eq!(
                            self.call_graph.add_edge(self.this, other),
                            self.call_sites.push(location)
                        );
                    }
                }
            } else {
                panic!("what could it be? {}", ty)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::init_logger;
    use rustc_data_structures::graph::scc::Sccs;

    #[test]
    fn test() {
        init_logger();
        compiler_interface::run_compiler_with_struct_defs_and_funcs(
            TEST_PROGRAMS.into(),
            |tcx, _, funcs| {
                let funcs = funcs
                    .into_iter()
                    .map(|did| did.to_def_id())
                    .collect::<Vec<_>>();
                let call_graph = CallGraph::new(tcx, funcs.into_iter());
                for caller in call_graph.graph.nodes() {
                    for callee in call_graph.successors(caller) {
                        let caller = call_graph.functions[caller];
                        let callee = call_graph.functions[callee];
                        log::debug!("{:?} ---> {:?}", caller, callee)
                    }
                }
                // h calls cond twice, so there should be two edges
                let mut cond_cnt = 0;
                for func in call_graph.successors(Func::from(2u32)) {
                    if func.index() == 5 {
                        cond_cnt += 1;
                    }
                }
                assert_eq!(cond_cnt, 2);
                let call_graph_sccs = Sccs::<Func, usize>::new(&call_graph);
                assert_eq!(call_graph_sccs.num_sccs(), 4);
                // l and f are not in the same component
                assert_ne!(
                    call_graph_sccs.scc(Func::from(0u32)),
                    call_graph_sccs.scc(Func::from(4u32))
                );
                // f, g, h are in the same component
                assert_eq!(
                    call_graph_sccs.scc(Func::from(0u32)),
                    call_graph_sccs.scc(Func::from(1u32))
                );
                assert_eq!(
                    call_graph_sccs.scc(Func::from(1u32)),
                    call_graph_sccs.scc(Func::from(2u32))
                );
                // m and f are not in the same component
                assert_ne!(
                    call_graph_sccs.scc(Func::from(0u32)),
                    call_graph_sccs.scc(Func::from(3u32))
                );
                let mut scc_nodes = vec![Vec::new(); call_graph_sccs.num_sccs()];
                for func in call_graph.graph.nodes() {
                    scc_nodes[call_graph_sccs.scc(func)].push(func)
                }
                for scc_node in scc_nodes {
                    log::debug!(
                        "Scc component: {:?}",
                        scc_node
                            .iter()
                            .map(|&func| call_graph.functions[func])
                            .collect::<Vec<_>>()
                    )
                }
            },
        )
    }

    const TEST_PROGRAMS: &'static str = "
        fn f() {
            assert!(true);
            if cond() {
                g()
            }
        }
        fn g() {
            h()
        }
        fn h() {
            if !cond() {
                f();
            }
            if cond() {
                g();
            }
            l()
        }
        fn m() {
            g()
        }
        fn l() {}
        #[inline(never)]
        fn cond() -> bool {
            true
        }
        ";
}
