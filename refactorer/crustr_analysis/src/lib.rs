#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(iter_zip)]
#![feature(crate_visibility_modifier)]
#![feature(maybe_uninit_extra)]

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind::FnDef};

pub mod interprocedural;
pub mod ownership_analysis;
pub mod pointer_analysis;
pub mod toy_analysis;

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;

use graph::{
    implementation::sparse_bit_vector::SparseBitVectorGraph, DirectedGraph, GraphSuccessors,
    WithNumNodes, WithSuccessors,
};
use index::{bit_set::HybridIter, vec::IndexVec};
use rustc_middle::mir::TerminatorKind;

index::newtype_index! {
    pub struct Function {
        DEBUG_FORMAT = "Function({})"
    }
}

pub struct FunctionData {
    pub def_id: DefId,
}

impl From<DefId> for FunctionData {
    fn from(def_id: DefId) -> Self {
        FunctionData { def_id }
    }
}

pub struct CallGraph<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    /// Invariant: functions are sorted by def_id
    pub functions: IndexVec<Function, FunctionData>,
    pub callers: SparseBitVectorGraph<Function>,
    callees: SparseBitVectorGraph<Function>,
}

impl<'tcx> CallGraph<'tcx> {
    #[inline]
    pub fn callees(&self) -> &SparseBitVectorGraph<Function> {
        &self.callees
    }

    pub fn lookup_def_id(&self, did: DefId) -> Option<Function> {
        self.functions
            .binary_search_by_key(&did, |data| data.def_id)
            .ok()
    }
}

impl<'tcx> DirectedGraph for CallGraph<'tcx> {
    type Node = Function;
}

impl<'tcx> WithNumNodes for CallGraph<'tcx> {
    fn num_nodes(&self) -> usize {
        self.functions.len()
    }
}

impl<'graph, 'tcx> GraphSuccessors<'graph> for CallGraph<'tcx> {
    type Item = Function;
    type Iter = HybridIter<'graph, Function>;
}

impl<'tcx> WithSuccessors for CallGraph<'tcx> {
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors>::Iter {
        self.callers.successors(node)
    }
}

pub struct CallGraphConstruction<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub functions: IndexVec<Function, FunctionData>,
}

impl<'tcx> CallGraphConstruction<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, bodies: &mut [DefId]) -> Self {
        bodies.sort();
        let functions = IndexVec::from_iter(bodies.iter().map(|&did| FunctionData { def_id: did }));
        CallGraphConstruction { tcx, functions }
    }

    pub fn lookup_def_id(&self, did: DefId) -> Option<Function> {
        self.functions
            .binary_search_by_key(&did, |data| data.def_id)
            .ok()
    }

    pub fn run(self) -> CallGraph<'tcx> {
        let mut graph = SparseBitVectorGraph::new(self.functions.len(), std::iter::empty());
        for (
            caller,
            &FunctionData {
                def_id: caller_def_id,
            },
        ) in self.functions.iter_enumerated()
        {
            let body = self.tcx.optimized_mir(caller_def_id);
            for terminator in body.basic_blocks().iter().map(|bb| bb.terminator()) {
                match &terminator.kind {
                    TerminatorKind::Call {
                        func,
                        args: _,
                        destination: _,
                        cleanup: _,
                        from_hir_call,
                        fn_span: _,
                    } => {
                        if let &FnDef(callee_def_id, generic_args) =
                            func.constant().unwrap().ty().kind()
                        {
                            if let Some(callee) = self.lookup_def_id(callee_def_id) {
                                assert!(
                                    generic_args.is_empty(),
                                    "Generic functions are not supported"
                                );
                                assert!(from_hir_call, "Inner functions are not supported");

                                graph.add_edge(caller, callee);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let rgraph = graph.reverse();

        CallGraph {
            tcx: self.tcx,
            functions: self.functions,
            callers: graph,
            callees: rgraph,
        }
    }
}

pub trait MirPass {
    type Output;
}
