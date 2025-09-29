use crate::Analysis;
use rustc_ast::NodeId;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_index::IndexVec;
use rustc_middle::middle::stability::Index;
use rustc_middle::{mir::Local, ty::TyCtxt};
use smallvec::SmallVec;
use utils::{dsa::fixed_shape::VecVec, rustc::RustProgram};

const MAX_PTR_PRECISION: usize = 1; // Max precision is 3 for pointer analysis

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PtrKind {
    OptMutRef, // output parameter: Option<&mut T>
    MutRef,    // mutable reference: &mut T
    MutRaw,    // mutable raw pointer: *mut T
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PtrKindDiff {
    pub before: PtrKind,
    pub after: PtrKind,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SigDecision {
    pub input_decs: Vec<Option<PtrKind>>, // None means no change
    pub output_dec: Option<PtrKind>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SigDecisions {
    data: FxHashMap<DefId, SigDecision>,
}

impl SigDecisions {
    pub fn expect(&self, did: &DefId) -> &SigDecision {
        self.data.get(did).unwrap()
    }

    pub fn new(rust_program: &RustProgram, analysis: &Analysis) -> Self {
        let mut data = FxHashMap::default();
        data.reserve(rust_program.functions.len());

        for did in rust_program.functions.iter() {
            let output_params = analysis.output_param_result.get(did).unwrap();
            let promoted_mut_refs = analysis.promoted_mut_ref_result.get(did).unwrap();

            let body = &*rust_program
                .tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();

            let input_decs = body
                .args_iter()
                .map(|param| {
                    if output_params.contains(param) {
                        Some(PtrKind::OptMutRef)
                    } else if promoted_mut_refs.contains(param) {
                        Some(PtrKind::MutRef)
                    } else {
                        None
                    }
                })
                .collect();

            data.insert(
                *did,
                SigDecision {
                    input_decs,
                    output_dec: None, // Currently intra-procedural borrow inference
                },
            );
        }
        SigDecisions { data }
    }
}

pub struct Decisions {
    did_idx: FxHashMap<DefId, usize>,
    data: VecVec<SmallVec<[Option<PtrKind>; MAX_PTR_PRECISION]>>, // Max precision is 1 for borrow inference
}

pub struct FnLocalDecisions(Decisions);

impl FnLocalDecisions {
    pub fn local_data(&self, did: &DefId) -> &[SmallVec<[Option<PtrKind>; MAX_PTR_PRECISION]>] {
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
                let mut local_decision: SmallVec<[Option<PtrKind>; MAX_PTR_PRECISION]> =
                    SmallVec::with_capacity(MAX_PTR_PRECISION);
                let is_output_param = output_params.contains(local);
                let is_promoted_mut_ref = promoted_mut_refs.contains(local);

                // is_output_param -> is_promoted_mut_ref
                assert!(!is_output_param || is_promoted_mut_ref);

                if local.index() == 0 {
                    // Currently intra-procedural borrow inference
                    local_decision.push(None);
                } else if is_output_param {
                    local_decision.push(Some(PtrKind::OptMutRef));
                } else if is_promoted_mut_ref {
                    local_decision.push(Some(PtrKind::MutRef));
                } else {
                    local_decision.push(None);
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
