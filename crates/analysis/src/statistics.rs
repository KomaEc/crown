use common::CrateData;
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    Body, Location,
};
use serde::{Deserialize, Serialize};

use crate::{
    ownership::solidify::SolidifiedOwnershipSchemes,
    type_qualifier::flow_insensitive::{fatness::FatnessResult, mutability::MutabilityResult},
};

#[derive(Default, Serialize, Deserialize)]
pub struct CrateStatistics {
    pub num_unsafe_ptrs: usize,
    pub num_non_arr_unsafe_ptrs: usize,
    pub num_mut_unsafe_ptrs: usize,
    pub num_non_arr_mut_unsafe_ptrs: usize,
    pub num_unsafe_usages: usize,
    pub num_non_arr_unsafe_usages: usize,
    pub num_mut_unsafe_usages: usize,
    pub num_non_arr_mut_unsafe_usages: usize,
    pub num_owning_ptrs_detected: usize,
}

impl CrateStatistics {
    pub fn new(
        crate_data: &CrateData,
        fatness_result: &FatnessResult,
        mutability_result: &MutabilityResult,
        ownership_result: &SolidifiedOwnershipSchemes,
    ) -> Self {
        let mut statistics = CrateStatistics::default();

        let tcx = crate_data.tcx;
        for &did in &crate_data.fns {
            let body = tcx.optimized_mir(did);

            // gather ptr count
            for (local, local_decl) in body.local_decls.iter_enumerated() {
                if local_decl.is_user_variable() && local_decl.ty.is_unsafe_ptr() {
                    let is_owning =
                        ownership_result.fn_results(&did).local_result(local)[0].is_owning();
                    let is_mutable =
                        mutability_result.fn_results(&did).local_result(local)[0].is_mutable();
                    let is_thin = fatness_result.fn_results(&did).local_result(local)[0].is_ptr();
                    if is_mutable && is_thin {
                        statistics.num_non_arr_mut_unsafe_ptrs += 1;
                        statistics.num_mut_unsafe_ptrs += 1;
                        statistics.num_non_arr_unsafe_ptrs += 1;
                    } else if is_mutable {
                        statistics.num_mut_unsafe_ptrs += 1;
                    } else {
                        statistics.num_non_arr_unsafe_ptrs += 1;
                    }
                    statistics.num_unsafe_ptrs += 1;
                    if is_owning {
                        statistics.num_owning_ptrs_detected += 1;
                    }
                }
            }

            // gather unsafe usages count
            CountUnsafeUsages {
                body: &body,
                fatness_result,
                mutability_result,
                statistics: &mut statistics,
            }
            .visit_body(&body);
        }

        statistics
    }
}

struct CountUnsafeUsages<'me, 'tcx> {
    body: &'me Body<'tcx>,
    fatness_result: &'me FatnessResult,
    mutability_result: &'me MutabilityResult,
    statistics: &'me mut CrateStatistics,
}

impl<'me, 'tcx> Visitor<'tcx> for CountUnsafeUsages<'me, 'tcx> {
    fn visit_place(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        context: PlaceContext,
        _: Location,
    ) {
        if matches!(context, PlaceContext::NonUse(..)) {
            return;
        }
        if self.body.local_decls[place.local].is_user_variable()
            && place.is_indirect()
            && self.body.local_decls[place.local].ty.is_unsafe_ptr()
        {
            let did = self.body.source.def_id();
            let local = place.local;
            let is_mutable =
                self.mutability_result.fn_results(&did).local_result(local)[0].is_mutable();
            let is_thin = self.fatness_result.fn_results(&did).local_result(local)[0].is_ptr();
            if is_mutable && is_thin {
                self.statistics.num_non_arr_mut_unsafe_usages += 1;
                self.statistics.num_mut_unsafe_usages += 1;
                self.statistics.num_non_arr_unsafe_usages += 1;
            } else if is_mutable {
                self.statistics.num_mut_unsafe_usages += 1;
            } else if is_thin {
                self.statistics.num_non_arr_unsafe_usages += 1;
            }
            self.statistics.num_unsafe_usages += 1;
        }
    }
}
