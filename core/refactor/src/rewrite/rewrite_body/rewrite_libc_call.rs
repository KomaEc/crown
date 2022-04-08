use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis,
    ssa::RichLocation, ty_ext::TyExt, LocationMap,
};
use either::Either;
use rewriter::Rewriter;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_index::{
    bit_set::BitSet,
    vec::{Idx, IndexVec},
};
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Local, Location, Operand, Place, ProjectionElem, Rvalue,
        Statement, StatementKind, Terminator, TerminatorKind, VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};

use crate::rewrite::rewrite_body::rewrite_use;

/// return: whether it requires further rewrite of LHS
pub fn rewrite_libc_call<'tcx>(
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
    destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> bool {
    let foreign_item = tcx.hir().expect_foreign_item(callee_did.expect_local());
    match foreign_item.ident.as_str() {
        "printf" => false,
        "calloc" => todo!(),
        "realloc" => todo!(),
        "malloc" => rewrite_malloc(
            tcx,
            rewriter,
            body,
            caller,
            ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            user_vars,
            names,
            args,
            destination,
            fn_span,
            location,
            editted_locations,
        ),
        "free" => rewrite_free(
            tcx,
            rewriter,
            body,
            caller,
            ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            user_vars,
            names,
            args,
            destination,
            fn_span,
            location,
            editted_locations,
        ),
        "memmove" => todo!(),
        "memcpy" => todo!(),
        "memset" => todo!(),
        "strncat" => todo!(),
        "strcmp" => todo!(),
        "strstr" => todo!(),
        "strlen" => todo!(),
        s => panic!("extern call to {s} is not supported"),
    }
}

fn rewrite_malloc<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    caller: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    args: &Vec<Operand<'tcx>>,
    destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> bool {
    let ([arg], _) = args.split_array_ref();
    if let Some(arg) = arg.place() {
        let arg = arg
            .as_local()
            .expect("arguments are assumed to be temporaries");
        assert!(
            !user_vars.contains(arg),
            "arguments are assumed to be temporaries"
        );
        let source_map = &fatness_analysis.ssa_name_source_map[caller];
        let fatness_ssa_idx = source_map.try_use(arg, location).unwrap();
        let def_rich_location = &fatness_analysis.def_sites[caller].defs[arg][fatness_ssa_idx];
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
    } else {
        unreachable!("arguments to malloc must be temporaries")
    }

    rewriter.make_suggestion(
        tcx,
        fn_span,
        "removing malloc".to_string(),
        "Some(Box::new(Default::default()))".to_string(),
    );

    let (dest, _) = destination.unwrap();

    return !user_vars.contains(dest.local);
    /*
    if !user_vars.contains(dest.local) {
        true
    } else {
        // assert!(false, "destination place is assumed to be a temporary in {:?}", terminator.kind)
        assert!(dest.projection.is_empty(), "destination place is assumed to be a local, and we need not rewrite")
    }
    */
}


fn rewrite_free<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    caller: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    args: &Vec<Operand<'tcx>>,
    destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> bool {
    let ([arg], _) = args.split_array_ref();
    if let Some(arg) = arg.place() {
        let arg = arg
            .as_local()
            .expect("arguments are assumed to be temporaries");
        assert!(
            !user_vars.contains(arg),
            "arguments are assumed to be temporaries"
        );
        let source_map = &fatness_analysis.ssa_name_source_map[caller];
        let fatness_ssa_idx = source_map.try_use(arg, location).unwrap();
        let def_rich_location = &fatness_analysis.def_sites[caller].defs[arg][fatness_ssa_idx];
        let def_location = match def_rich_location {
            &RichLocation::Mir(l) => l,
            &RichLocation::Phi(_) => todo!(),
            // we cannot end up in this branch, since
            // rhs is not user variable and must be initialised
            RichLocation::Entry => unreachable!(),
        };

        let arg_span = rewrite_use(
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
        ).unwrap();

        rewriter.make_suggestion(
            tcx,
            fn_span.shrink_to_lo().until(arg_span),
            "removing free prefix".to_string(),
            "let _ = ".to_string(),
        );

        rewriter.make_suggestion(
            tcx,
            arg_span.between(fn_span.shrink_to_hi()),
            "removing free suffix".to_string(),
            "".to_string(),
        );

        false
    } else {
        unreachable!("arguments to malloc must be temporaries")
    }


}