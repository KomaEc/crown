#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(split_array)]
#![feature(generic_associated_types)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(array_windows)]
#![feature(let_else)]
#![feature(let_chains)]
#![feature(is_some_with)]
#![feature(type_alias_impl_trait)]

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

pub mod analysis;
mod call_graph;
mod ptr;
mod rustc_properties;
mod struct_topology;
#[cfg(test)]
mod test;

use call_graph::CallGraph;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use struct_topology::{HasStructTopology, StructTopology};

/// Input program is assumed to consist of only top-level
/// functions and struct definitions.
pub struct CrateCtxt<'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: CallGraph,
    struct_topology: StructTopology,
}

impl<'tcx> From<orc_common::CrateData<'tcx>> for CrateCtxt<'tcx> {
    fn from(krate: orc_common::CrateData<'tcx>) -> Self {
        CrateCtxt {
            tcx: krate.tcx,
            call_graph: CallGraph::new(krate.tcx, &krate.fns),
            struct_topology: StructTopology::new(krate.tcx, krate.structs),
        }
    }
}

impl<'tcx> CrateCtxt<'tcx> {
    /// TODO: remove this
    pub fn new(tcx: TyCtxt<'tcx>, functions: Vec<DefId>, structs: Vec<DefId>) -> Self {
        CrateCtxt {
            tcx,
            call_graph: CallGraph::new(tcx, &functions[..]),
            struct_topology: StructTopology::new(tcx, structs),
        }
    }

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

impl<'tcx> HasStructTopology for CrateCtxt<'tcx> {
    #[inline]
    fn struct_topology(&self) -> &StructTopology {
        &self.struct_topology
    }
}
