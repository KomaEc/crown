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
pub mod noalias_params;
pub mod output_params;
pub mod ownership;
mod ptr;
mod rustc_properties;
pub mod ssa;
mod struct_topology;
#[cfg(test)]
mod test;

use alias::AliasResult;
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
    // taint_result: taint::TaintResult,
}

impl<'tcx> From<common::CrateData<'tcx>> for CrateCtxt<'tcx> {
    fn from(krate: common::CrateData<'tcx>) -> Self {
        CrateCtxt {
            tcx: krate.tcx,
            call_graph: CallGraph::new(krate.tcx, &krate.fns),
            struct_topology: StructTopology::new(krate.tcx, &krate.structs),
            // taint_result: taint::taint_results(&krate)
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

impl<'tcx> HasStructTopology for CrateCtxt<'tcx> {
    #[inline]
    fn struct_topology(&self) -> &StructTopology {
        &self.struct_topology
    }
}

pub fn show_param_qualifiers(crate_ctxt: &CrateCtxt, alias_result: &AliasResult) {
    for &did in crate_ctxt.fns() {
        let body = crate_ctxt.tcx.optimized_mir(did);
        let output_params = output_params::least_output_params(body, crate_ctxt);
        let output_params_str = output_params
            .iter()
            .map(|local| format!("{:?}", local))
            .collect::<Vec<_>>()
            .join(", ");

        let noalias_params = noalias_params::conservative_noalias_params(body, alias_result);
        let noalias_params_str = noalias_params
            .iter()
            .map(|local| format!("{:?}", local))
            .collect::<Vec<_>>()
            .join(", ");

        let unique_params = output_params.intersection(&noalias_params);
        let unique_params_str = unique_params
            .map(|local| format!("{:?}", local))
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "@{}: output_params: {output_params_str}, noalias_params: {noalias_params_str}, unique_params: {unique_params_str}",
            crate_ctxt.tcx.def_path_str(did)
        )
    }
}
