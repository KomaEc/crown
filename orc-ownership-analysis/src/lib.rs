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
#![feature(is_some_with)]
#![feature(type_alias_impl_trait)]
// #![feature(allocator_api)]

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
pub mod call_graph;
mod rustc_properties;
mod struct_topology;
#[cfg(test)]
mod test;
pub mod ptr;

use call_graph::CallGraph;
use orc_common::OrcInput;
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

impl<'tcx> OrcInput<'tcx> for CrateCtxt<'tcx> {
    #[inline]
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    #[inline]
    fn functions(&self) -> &[DefId] {
        self.call_graph.functions()
    }

    #[inline]
    fn structs(&self) -> &[DefId] {
        self.structs()
    }
}

impl<'tcx> CrateCtxt<'tcx> {
    pub fn from_input<Input: OrcInput<'tcx>>(input: &Input) -> Self {
        CrateCtxt {
            tcx: input.tcx(),
            call_graph: CallGraph::new(input.tcx(), input.functions()),
            struct_topology: StructTopology::new(input.tcx(), input.structs().to_vec()),
        }
    }

    /// TODO: remove this
    pub fn new(tcx: TyCtxt<'tcx>, functions: Vec<DefId>, structs: Vec<DefId>) -> Self {
        CrateCtxt {
            tcx,
            call_graph: CallGraph::new(tcx, &functions[..]),
            struct_topology: StructTopology::new(tcx, structs),
        }
    }

    // #[inline]
    // fn call_graph(&self) -> &CallGraph {
    //     &self.call_graph
    // }

    // #[inline]
    // fn struct_topology(&self) -> &StructTopology {
    //     &self.struct_topology
    // }

    #[inline]
    pub fn functions(&self) -> &[DefId] {
        self.call_graph.functions()
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
