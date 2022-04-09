use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis, ssa::RichLocation,
};
use rewriter::Rewriter;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{BasicBlock, Body, Local, Location, Operand, Place},
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};

use crate::rewrite::rewrite_body::rewrite_use;

pub fn rewrite_library_call<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    caller: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    callee_did: DefId,
    args: &Vec<Operand<'tcx>>,
    _destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) {
    let def_path = tcx.def_path(callee_did);

    // if it is a library call in core::ptr
    if def_path
        .data
        .get(0)
        .map(|d| match d.data {
            rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
            _ => false,
        })
        .is_some()
    {
        // if it is core::ptr::<..>::..
        if let Some(d) = def_path.data.get(3) {
            match d.data {
                // if it is core::ptr::<..>::offset
                rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "offset" => {
                    // self.model_ptr_offset(args, destination, location);
                    todo!()
                }
                // if it is core::ptr::<..>::is_null
                rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                    assert_eq!(args.len(), 1);
                    let ([ptr], _) = args.split_array_ref();
                    let ptr = ptr
                        .place()
                        .expect("argument to is_null should not be a constant");
                    let ptr = ptr
                        .as_local()
                        .expect("argument to is_null should be a temporary");
                    assert!(!user_vars.contains(ptr));
                    let source_map = &fatness_analysis.ssa_name_source_map[caller];
                    let fatness_ssa_idx = source_map.try_use(ptr, location).unwrap();
                    let def_rich_location =
                        &fatness_analysis.def_sites[caller].defs[ptr][fatness_ssa_idx];
                    let def_location = match def_rich_location {
                        &RichLocation::Mir(l) => l,
                        &RichLocation::Phi(_) => todo!(),
                        // we cannot end up in this branch, since
                        // rhs is not user variable and must be initialised
                        RichLocation::Entry => unreachable!(),
                    };
                    let _ = rewrite_use(
                        tcx,
                        rewriter,
                        body,
                        caller,
                        ownership_analysis,
                        mutability_analysis,
                        fatness_analysis,
                        user_vars,
                        names,
                        def_location,
                        editted_locations,
                    );
                    rewriter.make_suggestion(
                        tcx,
                        fn_span,
                        "refactor is_null to is_none".to_owned(),
                        "is_none()".to_owned(),
                    );
                }
                _ => return,
            }
        }
    }
}
