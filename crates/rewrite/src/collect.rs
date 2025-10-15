use crate::{
    Analysis,
    decision::{PtrKind, PtrKindDiff},
};
use rustc_data_structures::fx::FxHashSet;
use rustc_hash::FxHashMap;
use rustc_hir::{
    ExprKind, HirId, QPath, TyKind,
    def::{DefKind, Res},
    def_id::DefId,
    intravisit::{Visitor, walk_expr},
};
use rustc_middle::mir::Local;
use utils::{ir_util::map_thir_to_mir, rustc::RustProgram};

pub fn collect_diffs<'tcx>(
    rust_program: &RustProgram<'tcx>,
    analysis: &Analysis,
) -> FxHashMap<HirId, PtrKindDiff> {
    // Res::Local(id) -> (PtrKind before rewrite, PtrKind after rewrite)
    let mut ptr_kind_diffs: FxHashMap<HirId, PtrKindDiff> = FxHashMap::default();

    let fn_ptrs = collect_fn_ptrs(rust_program);

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

        let used_as_fn_ptr = fn_ptrs.contains(did);
        let input_skip_len = rust_program
            .tcx
            .fn_sig(*did)
            .skip_binder()
            .inputs()
            .skip_binder()
            .len();

        for (local, decl) in body
            .local_decls
            .iter_enumerated()
            .skip(1 + input_skip_len * (used_as_fn_ptr as usize))
        // skip inputs if used as fn ptr
        {
            let mutability = decl.ty.is_mutable_ptr();
            let ptr_kind = if output_params.contains(local) {
                assert!(mutability == true); // output parameters are always &mut T
                PtrKind::OptRef(true)
            } else if promoted_mut_refs.contains(local) {
                PtrKind::OptRef(mutability)
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
                        before: PtrKind::Raw(mutability),
                        after: ptr_kind,
                    },
                );
            }
        }
    }

    ptr_kind_diffs
}

pub fn collect_fn_ptrs(rust_program: &RustProgram) -> FxHashSet<DefId> {
    struct FnPtrCollector {
        pub fn_ptrs: FxHashSet<DefId>,
    }

    impl<'tcx> Visitor<'tcx> for FnPtrCollector {
        fn visit_expr(&mut self, ex: &'tcx rustc_hir::Expr<'tcx>) -> Self::Result {
            if let ExprKind::Cast(inner, ty) = ex.kind
                && let TyKind::BareFn(_) = ty.kind
                && let ExprKind::Path(ref qpath) = inner.kind
                && let QPath::Resolved(_, path) = qpath
                && let Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) = path.res
            {
                self.fn_ptrs.insert(def_id);
            }
            walk_expr(self, ex);
        }
    }

    let mut collector = FnPtrCollector {
        fn_ptrs: FxHashSet::default(),
    };

    for def_id in rust_program.functions.iter() {
        let local_def_id = def_id.expect_local();
        let body = rust_program.tcx.hir_body_owned_by(local_def_id);
        collector.visit_body(body);
    }

    collector.fn_ptrs
}
