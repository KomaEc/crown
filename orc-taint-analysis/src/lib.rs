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

use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
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

pub fn taint_results(input: &orc_common::CrateData) -> Steensgaard {
    Steensgaard::new(input)
}

pub fn report_results(input: &orc_common::CrateData) {
    Steensgaard::new(input).print_results()
}

impl Steensgaard {
    pub fn aliasing_field_pairs(&self) -> FxHashMap<DefId, Vec<(usize, usize)>> {
        self.struct_fields
            .iter()
            .map(|(&did, fields)| {
                (did, {
                    let Range { start, end } = fields;
                    (start..end)
                        .flat_map(|f| ((f + 1u32)..end).map(move |g| (f, g)))
                        .filter_map(|(f, g)| {
                            self.may_alias(f, g)
                                .then(|| (f.index() - start.index(), g.index() - start.index()))
                        })
                        .collect()
                })
            })
            .collect()
    }

    pub fn maybe_owning_fields(&self) -> FxHashMap<DefId, Vec<usize>> {
        self.struct_fields
            .iter()
            .map(|(&did, fields)| {
                (did, {
                    let Range { start, end } = fields;
                    (start..end)
                        .filter_map(|f| {
                            self.may_alias(f, self.free_arg)
                                .then(|| f.index() - start.index())
                        })
                        .collect()
                })
            })
            .collect()
    }
}
