use crate::utils::graph_ext::implementation::forward_star;
use rustc_data_structures::graph::scc::Sccs;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Location, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;

pub(crate) struct CallGraph {
    // /// Invariant: the set of functions are sorted by `DefId` to facilitate
    // /// reverse lookup
    functions: IndexVec<Func, DefId>,
    // functions: FxHashMap<DefId, Func>,
    /// TODO
    call_sites: IndexVec<CallSite, ()>,
    graph: forward_star::Graph<Func, CallSite>,
    sccs_data: CallGraphSccsData,
}

struct CallGraphSccsData {
    sccs: Sccs<Func, CallGraphScc>,
    ranked_by_post_order: IndexVec<CallGraphScc, Vec<Func>>,
    // scc_in_post_order: Vec<usize>,
    // ranked_funcs: Vec<DefId>,
}

impl CallGraphSccsData {
    fn new(call_graph: &forward_star::Graph<Func, CallSite>) -> Self {
        let sccs = Sccs::new(&call_graph);
        // let mut scc_in_post_order = vec![0; sccs.num_sccs()+1];
        // for func in call_graph.nodes() {

        // }
        let mut ranked_by_post_order = IndexVec::from_elem_n(Vec::new(), sccs.num_sccs());
        for func in call_graph.nodes() {
            ranked_by_post_order[sccs.scc(func)].push(func);
        }
        CallGraphSccsData {
            sccs,
            ranked_by_post_order,
        }
    }
}

// delegate_graph_via!(CallGraph.graph: forward_star::Graph<Func, CallSite>);

impl CallGraph {
    pub(crate) fn new(tcx: TyCtxt, mut bodies: Vec<DefId>) -> Self {
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

    #[inline]
    pub fn functions(&self) -> &[DefId] {
        &self.functions.raw[..]
    }

    // #[inline]
    // fn lookup_function(&self, did: &DefId) -> Result<Func, Func> {
    //     self.functions.binary_search(did)
    // }
}

crate::macros::newtype_index! {
    pub(super) struct Func {
        DEBUG_FORMAT = "Function_({})"
    }
}

crate::macros::newtype_index! {
    pub struct CallSite {
        DEBUG_FORMAT = "CallSite_({})"
    }
}

crate::macros::newtype_index! {
    struct CallGraphScc {
        DEBUG_FORMAT = "FunctionGroup_({})"
    }
}

struct CallGraphConstruction<'tcx> {
    tcx: TyCtxt<'tcx>,
    functions: IndexVec<Func, DefId>,
    call_sites: IndexVec<CallSite, ()>,
    call_graph: forward_star::Graph<Func, CallSite>,
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
            sccs_data: CallGraphSccsData::new(&self.call_graph),
            graph: self.call_graph,
        }
    }
}

struct CallGraphNodeVis<'me, 'tcx> {
    tcx: TyCtxt<'tcx>,
    this: Func,
    functions: &'me IndexVec<Func, DefId>,
    call_sites: &'me mut IndexVec<CallSite, ()>,
    call_graph: &'me mut forward_star::Graph<Func, CallSite>,
}

impl<'me, 'tcx> Visitor<'tcx> for CallGraphNodeVis<'me, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator, location: Location) {
        if let TerminatorKind::Call {
            func,
            args: _,
            destination: _,
            target: _,
            cleanup: _,
            from_hir_call: _,
            fn_span: _,
        } = &terminator.kind
        {
            let Some(func_constant) = func.constant() else {
                tracing::warn!("closures or function pointers are now ignored");
                return
            };
            let ty = func_constant.ty();
            let &FnDef(callee_did, _generic_args) = ty.kind() else {
                panic!("what could it be? {}", ty)
            };
            // local defined functions: libc externs or user functions
            let Some(did) = callee_did.as_local() else { return };
            // if it is user functions
            if matches!(
                self.tcx.hir().find_by_def_id(did),
                Some(rustc_hir::Node::Item(_))
            ) {
                let other = self.functions.binary_search(&callee_did).unwrap();
                assert_eq!(
                    self.call_graph.add_edge(self.this, other),
                    self.call_sites.push(())
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::init_logger;
    use rustc_data_structures::graph::{scc::Sccs, WithSuccessors};

    #[test]
    fn test() {
        init_logger();
        crate::test::run_compiler_with(TEST_PROGRAMS.into(), |program| {
            let crate::CrateInfo {
                tcx: _,
                call_graph,
                struct_topology: _,
            } = program;
            for caller in call_graph.graph.nodes() {
                for callee in call_graph.graph.successors(caller) {
                    let caller = call_graph.functions[caller];
                    let callee = call_graph.functions[callee];
                    tracing::debug!("{:?} ---> {:?}", caller, callee)
                }
            }
            // h calls cond twice, so there should be two edges
            let mut cond_cnt = 0;
            for func in call_graph.graph.successors(Func::from(2u32)) {
                if func.index() == 5 {
                    cond_cnt += 1;
                }
            }
            assert_eq!(cond_cnt, 2);
            let call_graph_sccs = Sccs::<Func, usize>::new(&call_graph.graph);
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
                tracing::debug!(
                    "Scc component: {:?}",
                    scc_node
                        .iter()
                        .map(|&func| call_graph.functions[func])
                        .collect::<Vec<_>>()
                )
            }
        })
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
