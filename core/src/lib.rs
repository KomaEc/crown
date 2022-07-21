#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(split_array)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(array_windows)]
#![feature(let_else)]

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_type_ir;

mod analysis;
pub mod rewriter;
#[cfg(test)]
pub(crate) mod test;
mod macros;

use analysis::call_graph::CallGraph;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

/// Input program is assumed to consist of only top-level
/// functions and struct definitions.
pub struct Program<'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: CallGraph,
    structs: Vec<DefId>,
}

impl<'tcx> Program<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, functions: Vec<DefId>, structs: Vec<DefId>) -> Self {
        Program {
            tcx,
            call_graph: CallGraph::new(tcx, functions),
            structs,
        }
    }

    pub fn call_graph(&self) -> &CallGraph {
        &self.call_graph
    }
}
