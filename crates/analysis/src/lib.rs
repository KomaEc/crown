#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(split_array)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(array_windows)]
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
pub mod ownership;
mod ptr;
pub mod ssa;
mod struct_topology;
#[cfg(test)]
mod test;
pub mod type_qualifier;

use call_graph::CallGraph;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use struct_topology::StructTopology;

pub struct CrateCtxt<'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: CallGraph,
    struct_topology: StructTopology<'tcx>,
}

impl<'tcx> From<common::CrateData<'tcx>> for CrateCtxt<'tcx> {
    fn from(krate: common::CrateData<'tcx>) -> Self {
        CrateCtxt {
            tcx: krate.tcx,
            call_graph: CallGraph::new(krate.tcx, &krate.fns),
            struct_topology: StructTopology::new(krate.tcx, &krate.structs),
        }
    }
}

impl<'tcx> CrateCtxt<'tcx> {
    #[inline]
    pub fn fns(&self) -> &[DefId] {
        self.call_graph.fns()
    }

    #[inline]
    /// Return the set of top-level struct definitions in post order
    pub fn structs(&self) -> &[DefId] {
        self.struct_topology.structs_in_post_order()
    }
}
