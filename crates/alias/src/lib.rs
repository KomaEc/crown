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
#![feature(array_windows)]

use common::data_structure::vec_vec::VecVec;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use steensgaard::{
    FieldFocused, FieldInsensitive, InterProcedural, IntraProcedural, MergeDeallocArg,
    NopDeallocArg, Steensgaard,
};

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

mod steensgaard;

pub type TaintResult = Steensgaard<FieldFocused, MergeDeallocArg, InterProcedural>;
pub type AliasResult = Steensgaard<FieldInsensitive, NopDeallocArg, InterProcedural>;
pub type IntraAliasResult = Steensgaard<FieldInsensitive, NopDeallocArg, IntraProcedural>;

pub fn taint_results(input: &common::CrateData) -> TaintResult {
    Steensgaard::field_based(input)
}

pub fn alias_results(input: &common::CrateData) -> AliasResult {
    Steensgaard::field_insensitive(input)
}

pub fn intra_alias_results(input: &common::CrateData) -> IntraAliasResult {
    Steensgaard::field_insensitive(input)
}

pub fn report_results(input: &common::CrateData) {
    Steensgaard::<FieldFocused, MergeDeallocArg, InterProcedural>::field_based(input)
        .print_results()
}

impl TaintResult {
    pub fn aliasing_field_pairs(&self) -> FxHashMap<DefId, Vec<(usize, usize)>> {
        self.struct_fields
            .did_idx
            .iter()
            .map(|(&did, &idx)| {
                let fields = &self.struct_fields.locations[idx];
                let (start, end) = (
                    fields.first().copied().unwrap(),
                    fields.last().copied().unwrap(),
                );
                (
                    did,
                    (start..end)
                        .flat_map(|f| ((f + 1u32)..end).map(move |g| (f, g)))
                        .filter_map(|(f, g)| {
                            self.may_alias(f, g)
                                .then(|| (f.index() - start.index(), g.index() - start.index()))
                        })
                        .collect(),
                )
            })
            .collect()
    }

    pub fn fields_aliases(&self, did: &DefId) -> VecVec<usize> {
        let idx = self.struct_fields.did_idx[did];
        let fields_locations = &self.struct_fields.locations[idx];
        let mut aliases = VecVec::with_indices_capacity(fields_locations.len());
        let start = fields_locations.first().copied().unwrap();
        for &[f_start, f_end] in fields_locations.array_windows() {
            assert_eq!(f_start + 1usize, f_end);
            let f = f_start;
            for &[g_start, g_end] in fields_locations.array_windows() {
                assert_eq!(g_start + 1usize, g_end);
                let g = g_start;
                if f == g {
                    continue;
                }
                if self.may_alias(f, g) {
                    aliases.push_inner(g.index() - start.index());
                }
            }
            aliases.push();
        }
        aliases.done()
    }

    pub fn maybe_owning_fields(&self) -> FxHashMap<DefId, Vec<usize>> {
        self.struct_fields
            .did_idx
            .iter()
            .map(|(&did, &idx)| {
                (did, {
                    let fields = &self.struct_fields.locations[idx];
                    let (start, end) = (
                        fields.first().copied().unwrap(),
                        fields.last().copied().unwrap(),
                    );
                    (start..end)
                        .filter_map(|f| {
                            self.may_alias(f, self.dealloc_arg)
                                .then(|| f.index() - start.index())
                        })
                        .collect()
                })
            })
            .collect()
    }
}
