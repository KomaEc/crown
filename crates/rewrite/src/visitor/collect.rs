use crate::{
    Analysis,
    decision::{PtrKind, PtrKindDiff, SigDecisions},
};
use itertools::izip;
use rustc_hash::FxHashMap;
use rustc_hir::{ExprKind, HirId, ItemId, def::Res};
use rustc_hir::{ItemKind, intravisit, intravisit::Visitor as HirVisitor};
use rustc_middle::mir::Local;
use utils::ir_util::IrMappings;
use utils::{ir_util::map_thir_to_mir, rustc::RustProgram};

// 1. collect each HIR variable(from MIR Local)'s before and after
// 4. collect HIR variable (that will be rewritten) usage sites
// 5. collect each HIR variable's usage sites as HirID
// 5-1. collect is_null checks for Option<&mut T>

pub fn collect_diffs<'tcx, 'a>(
    rust_program: &'tcx RustProgram<'tcx>,
    analysis: &Analysis,
) -> FxHashMap<HirId, PtrKindDiff> {
    // Res::Local(id) -> (PtrKind before rewrite, PtrKind after rewrite)
    let mut ptr_kind_diffs: FxHashMap<HirId, PtrKindDiff> = FxHashMap::default();

    // 1. collect each HIR variable's before pointer kinds (and after pointer kinds for output params and promoted mut refs)
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

// struct UsageCollector<'tcx, 'a> {
//     rust_program: &'tcx RustProgram<'tcx>,
//     usages: &'a mut FxHashMap<HirId, Vec<HirId>>, // Res::Local(id) -> usage sites
// }

// struct FnCallCollector<'tcx, 'a> {
//     rust_program: &'tcx RustProgram<'tcx>,
//     fn_sig_decs: &'a SigDecisions,
// }

// impl<'tcx> HirVisitor<'tcx> for FnCallCollector<'tcx, '_> {
//     fn visit_expr(&mut self, ex: &'tcx rustc_hir::Expr<'tcx>) -> Self::Result {
//         let typeck = self.rust_program.tcx.typeck(ex.hir_id.owner);

//         if let ExprKind::Call(func, args) = ex.kind
//             && let ExprKind::Path(fn_qpath) = func.kind  // get function being called
//             && let Res::Def(_, fn_did) = typeck.qpath_res(&fn_qpath, func.hir_id)
//         {
//             let fn_sig_dec = self.fn_sig_decs.expect(&fn_did);

//             for (arg, input_dec) in izip!(args.iter(), fn_sig_dec.input_decs.iter()) {
//                 let ExprKind::Path(arg_qpath) = arg.kind else {
//                     continue;
//                 };
//                 match input_dec {
//                     None | Some(PtrKind::MutRaw) => continue, // no change
//                     Some(PtrKind::OptMutRef) => {}
//                     Some(PtrKind::MutRef) => {}
//                 }
//                 let arg_res = typeck.qpath_res(&arg_qpath, arg.hir_id);
//                 if let Res::Local(arg_hir_id) = arg_res {
//                     // arg_hir_id is passed to fn_did
//                 }
//             }
//         }
//         intravisit::walk_expr(self, ex);
//     }
// }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CollectorResult {
    pub fn_sig_decs: SigDecisions,
    pub ptr_kind_diffs: FxHashMap<HirId, PtrKindDiff>, // Res::Local(id) -> (PtrKind before rewrite, PtrKind after rewrite)
}
