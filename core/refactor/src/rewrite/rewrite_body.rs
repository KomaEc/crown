use analysis::{call_graph::Func, ownership_analysis};
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
    func: Func,
    required_mutability: &BitSet<Local>,
    fn_did: LocalDefId,
) {
    let body = tcx.optimized_mir(fn_did);
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

fn rewrite_terminator() {}
