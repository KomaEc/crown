//! Taint analysis for struct fields in a crate.
//!
//! Given a crate with a set of C struct definitions, this analysis
//! produces flow-insensitive results on whether a field `f` (of pointer
//! type) of struct `S` may flow into a field `g` of struct `T`.
//!
//! The taint analysis is essentially a field-based Steensgaard's analysis.
//! Struct fields initially point to pointer targets that represent those
//! fields. Assignment statements in the crate merge (directly or indirectly)
//! those targets. `S.f` may taint `T.g` if `S.f` and `T.g` are unified
//! in the analysis result.

#![feature(rustc_private)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]
#![feature(let_else)]

use orc_common::OrcInput;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::bit_set::HybridBitSet;
use rustc_middle::mir::Local;
use steensgaard::Steensgaard;

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
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

pub(crate) mod steensgaard;

// pub struct AddressTakenTargets {
//     locals: FxHashMap<DefId, HybridBitSet<Local>>,
//     fields: FxHashMap<DefId, HybridBitSet<usize>>,
// }

// impl AddressTakenTargets {
//     pub fn new<'tcx, Input: OrcInput<'tcx>>(input: Input) -> Self {
//         for did in input.functions() {

//         }
//     }
// }

pub fn run_steensgaard<'tcx, Input: OrcInput<'tcx>>(input: Input) {
    let _ = Steensgaard::new(input);
}
