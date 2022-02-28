use graph::{implementation::forward_star::{self, PredecessorNodes, SuccessorNodes}, derive_graph_via};
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
    /// `call_graph.edges` contains the universe of all call sites.
    /// Note that the order is fixed as long as a visitor traverse
    /// the MIR in the default way.
    pub call_graph: forward_star::Graph<Func, CallSite>,
}

derive_graph_via!(CallGraph.call_graph: forward_star::Graph<Func, CallSite>);

impl CallGraph {
    pub fn new(tcx: TyCtxt, bodies: impl Iterator<Item = DefId>) -> Self {
        let mut bodies = bodies.collect::<Vec<_>>();
        bodies.sort();
        let num_nodes = bodies.len();
        CallGraphConstruction {
            tcx,
            functions: IndexVec::from_raw(bodies),
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
                call_graph: &mut self.call_graph,
            }
            .visit_body(body);
        }
        CallGraph {
            functions: self.functions,
            call_graph: self.call_graph,
        }
    }
}

struct CallGraphNodeVis<'me, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub this: Func,
    pub functions: &'me IndexVec<Func, DefId>,
    pub call_graph: &'me mut forward_star::Graph<Func, CallSite>,
}

impl<'me, 'tcx> Visitor<'tcx> for CallGraphNodeVis<'me, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator, _location: Location) {
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
                        self.call_graph.add_edge(self.this, other);
                    }
                }
            }
        }
    }
}
