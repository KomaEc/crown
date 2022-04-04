use analysis::{call_graph::Func, fat_thin_analysis, ownership_analysis};
use rewriter::Rewriter;
use rustc_hir::def_id::LocalDefId;
use rustc_index::bit_set::BitSet;
use rustc_middle::{
    mir::{Local, VarDebugInfoContents},
    ty::TyCtxt,
};

pub fn rewrite_fn_body(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    required_mutability: &BitSet<Local>,
    fn_did: LocalDefId,
) {
    let body = tcx.optimized_mir(fn_did);

    rewrite_calls(
        tcx,
        rewriter,
        ownership_analysis,
        fatness_analysis,
        required_mutability,
        body
    );

    for var_debug in &body.var_debug_info {
        match var_debug.value {
            VarDebugInfoContents::Place(place) => {
                let local = place
                    .as_local()
                    .expect("user variable should be mapped to a local");
            }
            VarDebugInfoContents::Const(constant) => {
                log::warn!("user constant {:?} is not processed", constant)
            }
        }
    }
}

fn rewrite_calls(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    required_mutability: &BitSet<Local>,
    body: &rustc_middle::mir::Body<'_>,
) {
    for bb_data in body.basic_blocks().iter() {
        let terminator = bb_data.terminator();
        if let rustc_middle::mir::TerminatorKind::Call {
            ref func,
            ref args,
            destination,
            cleanup: _,
            from_hir_call: _,
            fn_span: _,
        } = terminator.kind
        {
            let ty = func
                .constant()
                .expect("closures or function pointers are not supported!")
                .ty();
            if let &rustc_middle::ty::TyKind::FnDef(callee_did, _generic_args) = ty.kind() {
                match callee_did.as_local() {
                    Some(did) => {
                        if matches!(
                            tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::ForeignItem(_))
                        ) {
                            // self.model_libc_call(callee_did, args, destination, location);
                        } else if matches!(
                            tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::Item(_))
                        ) {
                            // self.model_boundary(callee_did, args, destination, location);
                        }
                    }
                    None => {
                        // self.model_library_call(callee_did, args, destination, location);
                    }
                }
            }
        }
    }
}
