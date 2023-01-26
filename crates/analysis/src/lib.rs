#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(split_array)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(array_windows)]
#![feature(array_try_map)]
#![feature(type_alias_impl_trait)]

extern crate either;

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

mod call_graph;
mod lattice;
pub mod ownership;
mod ptr;
pub mod ssa;
pub mod statistics;
mod struct_ctxt;
#[cfg(test)]
mod test;
pub mod type_qualifier;
pub mod use_def;

use call_graph::CallGraph;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use struct_ctxt::StructCtxt;

pub struct CrateCtxt<'tcx> {
    tcx: TyCtxt<'tcx>,
    fn_ctxt: CallGraph,
    struct_ctxt: StructCtxt<'tcx>,
}

impl<'tcx> CrateCtxt<'tcx> {
    pub fn new(crate_data: &common::CrateData<'tcx>) -> Self {
        CrateCtxt {
            tcx: crate_data.tcx,
            fn_ctxt: CallGraph::new(crate_data.tcx, &crate_data.fns),
            struct_ctxt: StructCtxt::new(crate_data.tcx, &crate_data.structs),
        }
    }

    #[inline]
    pub fn fns(&self) -> &[DefId] {
        self.fn_ctxt.fns()
    }

    #[inline]
    /// Return the set of top-level struct definitions in post order
    pub fn structs(&self) -> &[DefId] {
        self.struct_ctxt.structs_in_post_order()
    }
}
