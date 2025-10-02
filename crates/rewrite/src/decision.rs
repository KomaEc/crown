use crate::Analysis;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use utils::rustc::RustProgram;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PtrKind {
    OptMutRef, // output parameter: Option<&mut T>
    MutRef,    // mutable reference: &mut T
    MutRaw,    // mutable raw pointer: *mut T
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PtrKindDiff {
    pub before: PtrKind, // ptrkind before change (required by surrounding context)
    pub after: PtrKind,  // ptrkind after change (should be adjusted to satisfy `before`)
}

impl Default for PtrKindDiff {
    fn default() -> Self {
        PtrKindDiff {
            before: PtrKind::MutRaw,
            after: PtrKind::MutRaw,
        }
    }
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
    pub fn expect(&self, did: &DefId) -> SigDecision {
        self.data.get(did).unwrap().clone()
    }

    pub fn get(&self, did: &DefId) -> Option<SigDecision> {
        self.data.get(did).cloned()
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

            let sig = rust_program.tcx.fn_sig(*did).skip_binder();
            let input_len = sig.inputs().skip_binder().len();

            let input_decs = body
                .args_iter()
                .take(input_len) // exclude variadic arguments
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
