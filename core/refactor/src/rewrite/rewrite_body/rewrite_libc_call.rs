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

use crate::rewrite::rewrite_body::{rewrite_mir_statement, rewrite_non_user_local_use};

use super::RewriteUseResult;

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
) -> RewriteUseResult {
    let foreign_item = tcx.hir().expect_foreign_item(callee_did.expect_local());
    match foreign_item.ident.as_str() {
        "printf" => RewriteUseResult::Done,
        "calloc" => rewrite_calloc(
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
        "realloc" => RewriteUseResult::Done, //todo!(),
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
) -> RewriteUseResult {
    let ([arg], _) = args.split_array_ref();
    if let Some(arg) = arg.place() {
        let _ = arg
            .as_local()
            .expect("arguments are assumed to be temporaries");
        assert!(
            !user_vars.contains(arg.local),
            "arguments are assumed to be temporaries"
        );
        let _ = rewrite_non_user_local_use(
            tcx,
            rewriter,
            body,
            caller,
            ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            user_vars,
            names,
            &arg,
            location,
            editted_locations,
        );
    } else {
        unreachable!("arguments to malloc must be temporaries")
    }

    let (dest, _) = destination.unwrap();

    // return !user_vars.contains(dest.local);
    if !user_vars.contains(dest.local) {
        // the type of malloc res cannot be determined
        // return the span of the whole malloc call
        // for instance, malloc(4 as u64)
        //               ^^^^^^^^^^^^^^^^
        RewriteUseResult::FromMalloc {
            malloc_span: fn_span,
        }
    } else {
        // assert!(false, "destination place is assumed to be a temporary in {:?}", terminator.kind)
        assert!(
            dest.projection.is_empty(),
            "destination place is assumed to be a local, and we need not rewrite"
        );
        /*
        rewriter.make_suggestion(
            tcx,
            fn_span,
            "removing malloc".to_string(),
            "Some(Box::new(Default::default()))".to_string(),
        );
        */
        RewriteUseResult::Done
    }
}

fn rewrite_calloc<'tcx>(
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
) -> RewriteUseResult {
    let ([num, size], _) = args.split_array_ref();

    let num = num
        .place()
        .expect("arguments are assumed to be temporaries");
    assert!(!user_vars.contains(num.local));
    let num_span = rewrite_non_user_local_use(
        tcx,
        rewriter,
        body,
        caller,
        ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        user_vars,
        names,
        &num,
        location,
        editted_locations,
    )
    .expect_rhs_span();

    let size = size
        .place()
        .expect("arguments are assumed to be temporaries");
    assert!(!user_vars.contains(size.local));
    let size_span = rewrite_non_user_local_use(
        tcx,
        rewriter,
        body,
        caller,
        ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        user_vars,
        names,
        &size,
        location,
        editted_locations,
    )
    .expect_rhs_span();

    let (dest, _) = destination.unwrap();

    // return !user_vars.contains(dest.local);
    if !user_vars.contains(dest.local) {

        /* 
        rewriter.make_suggestion(
            tcx,
            num_span.shrink_to_hi().between(size_span.shrink_to_hi()),
            "remove size of type argument".to_string(),
            "".to_string(),
        );
        */

        // the type of malloc res cannot be determined
        // return the call prefix
        // for instance, calloc(12 as u64, 4 as u64)
        //               ^^^^^^^
        RewriteUseResult::FromCalloc {
            num_span,
            calloc_span: fn_span,
        }
    } else {
        // assert!(false, "destination place is assumed to be a temporary in {:?}", terminator.kind)
        assert!(
            dest.projection.is_empty(),
            "destination place is assumed to be a local, and we need not rewrite"
        );
        RewriteUseResult::Done
    }
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
    _destination: Option<(Place<'tcx>, BasicBlock)>,
    fn_span: Span,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> RewriteUseResult {
    let ([arg], _) = args.split_array_ref();
    if let Some(arg) = arg.place() {
        let _ = arg
            .as_local()
            .expect("arguments are assumed to be temporaries");
        assert!(
            !user_vars.contains(arg.local),
            "arguments are assumed to be temporaries"
        );

        if let RewriteUseResult::LHSToBeRewritten { rhs_span: arg_span } =
            rewrite_non_user_local_use(
                tcx,
                rewriter,
                body,
                caller,
                ownership_analysis,
                mutability_analysis,
                fatness_analysis,
                user_vars,
                names,
                &arg,
                location,
                editted_locations,
            )
        {
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
        } else {
            unimplemented!()
        }

        RewriteUseResult::Done
    } else {
        unreachable!("arguments to malloc must be temporaries")
    }
}
