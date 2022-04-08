use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis,
    ssa::RichLocation, ty_ext::TyExt,
};
use either::Either;
use rewriter::Rewriter;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Local, Location, Operand, Place, ProjectionElem, Rvalue,
        Statement, StatementKind, TerminatorKind, VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};

pub fn rewrite_call<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    func: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    location: Location,
    call: &Operand<'tcx>,
    args: &Vec<Operand<'tcx>>,
    destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    terminator_span: Span,
) -> Option<Span> {
    let ty = call
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
                    todo!()
                    // self.model_libc_call(callee_did, args, destination, location);
                    // return;
                } else if matches!(
                    tcx.hir().find_by_def_id(did),
                    Some(rustc_hir::Node::Item(_))
                ) {
                    todo!()
                    // self.model_boundary(callee_did, args, destination, location);
                    // return;
                }

                unreachable!()
            }
            None => {
                todo!()
                // self.model_library_call(callee_did, args, destination, location);
                // return;
            }
        }
    }
    unreachable!()
}
