#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(split_array)]

use alias::{AliasResult, TaintResult};
use analysis::{
    ownership::{whole_program::WholeProgramResults, Ownership},
    type_qualifier::flow_insensitive::{
        fatness::{Fatness, FatnessResult},
        mutability::{Mutability, MutabilityResult},
    },
};
use common::{data_structure::vec_vec::VecVec, CrateData};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use smallvec::SmallVec;

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_passes;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

pub fn refactor<'tcx>(crate_data: &CrateData<'tcx>, analysis: &Analysis<'tcx>) {
    let struct_decision = StructDecision::new(crate_data, analysis);
    println!("{:?}", struct_decision);
}

pub struct Analysis<'tcx> {
    alias_result: AliasResult,
    taint_result: TaintResult,
    ownership_schemes: WholeProgramResults<'tcx>,
    mutability_result: MutabilityResult,
    fatness_result: FatnessResult,
}

impl<'tcx> Analysis<'tcx> {
    pub fn new(
        alias_result: AliasResult,
        taint_result: TaintResult,
        ownership_schemes: WholeProgramResults<'tcx>,
        mutability_result: MutabilityResult,
        fatness_result: FatnessResult,
    ) -> Self {
        Analysis {
            alias_result,
            taint_result,
            ownership_schemes,
            mutability_result,
            fatness_result,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PointerKind {
    Move,
    Mut,
    Raw,
}

pub struct PointerData {
    pointer_kind: PointerKind,
    meta_data: MetaData,
}

pub struct MetaData {
    ownership: Ownership,
    mutability: Mutability,
    fatness: Fatness,
}

pub struct StructDecision {
    did_idx: FxHashMap<DefId, usize>,
    struct_fields: VecVec<SmallVec<[PointerData; 3]>>,
}

impl StructDecision {
    pub fn fields_data(&self, did: &DefId) -> &[SmallVec<[PointerData; 3]>] {
        let idx = self.did_idx[did];
        &self.struct_fields[idx]
    }
}

impl StructDecision {
    pub fn new<'tcx>(crate_data: &CrateData<'tcx>, analysis: &Analysis<'tcx>) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut struct_fields = VecVec::with_capacity(
            crate_data.structs.len(),
            crate_data.structs.iter().fold(0, |acc, did| {
                let adt_def = crate_data.tcx.adt_def(*did);
                acc + adt_def.all_fields().count()
            }),
        );

        for (idx, did) in crate_data.structs.iter().enumerate() {
            let fields_ownership = analysis.ownership_schemes.fields(did).collect::<Vec<_>>();
            let fields_mutability = analysis.mutability_result.struct_results(did);
            let fields_fatness = analysis.fatness_result.struct_results(did);
            let fields_aliases = analysis.taint_result.fields_aliases(did);

            for (ownership, mutability, fatness, aliases) in itertools::izip!(
                fields_ownership.iter().copied(),
                fields_mutability,
                fields_fatness,
                fields_aliases.iter()
            ) {
                assert_eq!(ownership.len(), mutability.len());
                assert_eq!(mutability.len(), fatness.len());

                let aliasing_nonowning_field = aliases.iter().any(|&idx| {
                    fields_ownership[idx]
                        .iter()
                        .all(|ownership| !ownership.is_owning())
                });
                let mut field = SmallVec::with_capacity(ownership.len());

                for (&ownership, &mutability, &fatness) in
                    itertools::izip!(ownership, mutability, fatness)
                {
                    let meta_data = MetaData {
                        ownership,
                        mutability,
                        fatness,
                    };
                    let pointer_kind = if ownership.is_owning()
                        && !fatness.is_arr()
                        && !aliasing_nonowning_field
                    {
                        PointerKind::Move
                    } else {
                        PointerKind::Raw
                    };
                    field.push(PointerData {
                        pointer_kind,
                        meta_data,
                    });
                }
                struct_fields.push_inner(field);
            }

            struct_fields.push();

            did_idx.insert(*did, idx);
        }

        let struct_fields = struct_fields.done();
        StructDecision {
            did_idx,
            struct_fields,
        }
    }
}

impl std::fmt::Debug for StructDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (did, &idx) in self.did_idx.iter() {
            let mut index = 0;
            writeln!(f, "@{:?}: {{", did)?;
            for field in self.struct_fields[idx].iter() {
                let field_str = field
                    .iter()
                    .map(|pointer_data| format!("{:?}", pointer_data.pointer_kind))
                    .collect::<Vec<_>>()
                    .join(" ");

                writeln!(f, "   {index}: {field_str}")?;

                index += 1;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

pub struct FnDecision {}
