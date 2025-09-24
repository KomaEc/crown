use crate::Analysis;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use smallvec::SmallVec;
use utils::{dsa::fixed_shape::VecVec, rustc::RustProgram};

const MAX_PTR_PRECISION: usize = 1; // Max precision is 3 for pointer analysis

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PtrDecision {
    OptMutRef,
    MutRef,
    AsIs,
}

impl PtrDecision {
    pub fn is_mut(self) -> bool {
        matches!(self, PtrDecision::OptMutRef | PtrDecision::MutRef)
    }
}

pub struct Decisions {
    did_idx: FxHashMap<DefId, usize>,
    data: VecVec<SmallVec<[PtrDecision; MAX_PTR_PRECISION]>>, // Max precision is 1 for borrow inference
}

pub struct FnLocalDecisions(Decisions);

impl FnLocalDecisions {
    pub fn local_data(&self, did: &DefId) -> &[SmallVec<[PtrDecision; MAX_PTR_PRECISION]>] {
        let idx = self.0.did_idx[did];
        &self.0.data[idx]
    }

    pub fn new(rust_program: &RustProgram, analysis: &Analysis) -> Self {
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(rust_program.functions.len());
        let mut fn_local_decs = VecVec::with_capacity(
            rust_program.functions.len(),
            rust_program.functions.iter().fold(0, |acc, did| {
                let r#fn = &*rust_program
                    .tcx
                    .mir_drops_elaborated_and_const_checked(did.expect_local())
                    .borrow();
                acc + r#fn.local_decls.len()
            }),
        );

        for (idx, did) in rust_program.functions.iter().enumerate() {
            let output_params = analysis.output_param_result.get(did).unwrap();
            let promoted_mut_refs = analysis.promoted_mut_ref_result.get(did).unwrap();

            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();

            for (local, _local_decl) in body.local_decls.iter_enumerated() {
                let mut local_decision: SmallVec<[PtrDecision; MAX_PTR_PRECISION]> =
                    SmallVec::with_capacity(MAX_PTR_PRECISION);
                let is_output_param = output_params.contains(local);
                let is_promoted_mut_ref = promoted_mut_refs.contains(local);

                // is_output_param -> is_promoted_mut_ref
                assert!(!is_output_param || is_promoted_mut_ref);

                if local.index() == 0 {
                    // Currently intra-procedural borrow inference
                    local_decision.push(PtrDecision::AsIs);
                } else if is_output_param {
                    local_decision.push(PtrDecision::OptMutRef);
                } else if is_promoted_mut_ref {
                    local_decision.push(PtrDecision::MutRef);
                } else {
                    local_decision.push(PtrDecision::AsIs);
                }

                fn_local_decs.push_element(local_decision);
            }

            fn_local_decs.complete_cur_vec();

            did_idx.insert(*did, idx);
        }

        let fn_local_decs = fn_local_decs.complete();
        FnLocalDecisions(Decisions {
            did_idx,
            data: fn_local_decs,
        })
    }
}
