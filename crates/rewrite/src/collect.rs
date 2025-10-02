use crate::{
    Analysis,
    decision::{PtrKind, PtrKindDiff},
};
use rustc_hash::FxHashMap;
use rustc_hir::HirId;
use rustc_middle::mir::Local;
use utils::{ir_util::map_thir_to_mir, rustc::RustProgram};

pub fn collect_diffs<'tcx>(
    rust_program: &RustProgram<'tcx>,
    analysis: &Analysis,
) -> FxHashMap<HirId, PtrKindDiff> {
    // Res::Local(id) -> (PtrKind before rewrite, PtrKind after rewrite)
    let mut ptr_kind_diffs: FxHashMap<HirId, PtrKindDiff> = FxHashMap::default();

    // collect each HIR variable's before/after pointer kinds
    for did in rust_program.functions.iter() {
        let output_params = analysis.output_param_result.get(did).unwrap();
        let promoted_mut_refs = analysis.promoted_mut_ref_result.get(did).unwrap();
        // Assume every mir local has one or less corresponding hir id
        let hir_to_mir = map_thir_to_mir(did.expect_local(), false, rust_program.tcx);
        let local_to_binding: FxHashMap<Local, HirId> = hir_to_mir
            .binding_to_local
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect();

        let body = &*rust_program
            .tcx
            .mir_drops_elaborated_and_const_checked(did.expect_local())
            .borrow();

        for (local, decl) in body.local_decls.iter_enumerated() {
            let ptr_kind = if output_params.contains(local) {
                PtrKind::OptMutRef
            } else if promoted_mut_refs.contains(local) {
                PtrKind::MutRef
            } else {
                continue;
            };
            if let Some(hir_id) = local_to_binding.get(&local) {
                let ty = decl.ty;
                // Ensure output parameters and promoted mutable references are raw pointers
                assert!(
                    ty.is_raw_ptr(),
                    "Expected raw pointer type, got {:?} in {:?}",
                    ty,
                    decl
                );
                ptr_kind_diffs.insert(
                    *hir_id,
                    PtrKindDiff {
                        before: PtrKind::MutRaw,
                        after: ptr_kind,
                    },
                );
            }
        }
    }

    ptr_kind_diffs
}
