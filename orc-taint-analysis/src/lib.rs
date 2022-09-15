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

use orc_common::OrcInput;
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

pub fn taint_results<'tcx, Input: OrcInput<'tcx>>(input: Input) -> Steensgaard {
    Steensgaard::new(input)
}

pub fn report_results<'tcx, Input: OrcInput<'tcx>>(input: Input) {
    Steensgaard::new(input).print_results()
}

impl Steensgaard {
    pub fn maybe_self_referential_structs(&self) -> FxHashMap<DefId, Vec<(usize, usize)>> {
        let mut may_self_referential: FxHashMap<DefId, Vec<(usize, usize)>> = FxHashMap::default();
        for (&did, fields) in self.struct_fields.iter() {
            let Range { start, end } = fields;
            for f in start..end {
                for g in (f + 1u32)..end {
                    if self.may_alias(f, g) {
                        let aliased_pair = (f.index() - start.index(), g.index() - start.index());
                        match may_self_referential.entry(did) {
                            std::collections::hash_map::Entry::Occupied(mut o) => {
                                o.get_mut().push(aliased_pair);
                            }
                            std::collections::hash_map::Entry::Vacant(v) => {
                                v.insert(vec![aliased_pair]);
                            }
                        }
                    }
                }
            }
        }
        may_self_referential
    }

    pub fn maybe_owning_fields(&self) -> FxHashMap<DefId, Vec<usize>> {
        let mut maybe_owning_fields: FxHashMap<DefId, Vec<usize>> = FxHashMap::default();
        for (&did, fields) in self.struct_fields.iter() {
            let Range { start, end } = fields;
            for f in start..end {
                if self.may_alias(f, self.free_arg) {
                    let f = f.index() - start.index();
                    match maybe_owning_fields.entry(did) {
                        std::collections::hash_map::Entry::Occupied(mut o) => o.get_mut().push(f),
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(vec![f]);
                        }
                    }
                }
            }
        }
        maybe_owning_fields
    }
}
