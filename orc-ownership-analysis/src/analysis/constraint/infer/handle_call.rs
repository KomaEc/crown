use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;

use crate::analysis::{
    constraint::{infer::InferCtxt, Database, OwnershipSig},
    consume::Consume,
    AnalysisKind, FnSig, WholeProgram,
};

pub mod handle_libc;
pub mod handle_library;

pub trait HandleCall<'infercx, 'tcx: 'infercx, DB: Database + 'infercx>:
    AnalysisKind<'infercx> + Sized
{
    fn handle_call(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: DefId,
    );

    fn handle_inputs(
        // infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        inter_ctxt: &Self::InterCtxt,
        database: &mut DB,
        r#fn: DefId,
        inputs: impl Iterator<Item = Option<Range<OwnershipSig>>>,
    );

    fn handle_output(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        r#fn: DefId,
        output: Option<Range<OwnershipSig>>,
    );
}

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx, K: AnalysisKind<'infercx>>
    HandleCall<'infercx, 'tcx, DB> for K
{
    default fn handle_call(
        _: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        _: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        _: DefId,
    ) {
    }

    default fn handle_inputs(
        // _: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        // _: &CrateCtxt<'_>,
        _: &K::InterCtxt,
        _: &mut DB,
        _: DefId,
        _: impl Iterator<Item = Option<Range<OwnershipSig>>>,
    ) {
    }

    default fn handle_output(
        _: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        _: DefId,
        _: Option<Range<OwnershipSig>>,
    ) {
    }
}

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx> HandleCall<'infercx, 'tcx, DB>
    for WholeProgram
{
    fn handle_call(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: DefId,
    ) {
        let c_variadic = infer_cx
            .crate_ctxt
            .tcx
            .fn_sig(callee)
            .skip_binder()
            .c_variadic;

        let callee_sigs = &infer_cx.inter_ctxt[&callee];

        // println!("{:?}", callee_sigs);

        // #[cfg(debug_assertions)]
        // if !c_variadic {
        //     assert_eq!(callee_sigs.iter().count(), caller.iter().count());
        // }

        let mut callee_caller = callee_sigs.iter().zip(caller.iter());

        // dest = ret ~> rho(dest) = 0, rho(dest') = rho(ret)
        let (ret, dest) = callee_caller.next().unwrap();
        if let Some(ret) = ret.clone() {
            if let Some(Consume {
                r#use: dest_use,
                def: dest_def,
            }) = dest.clone()
            {
                for (ret, dest_use, dest_def) in izip!(ret, dest_use, dest_def) {
                    infer_cx
                        .database
                        .push_assume::<crate::analysis::constraint::Debug>((), dest_use, false);
                    infer_cx
                        .database
                        .push_equal::<crate::analysis::constraint::Debug>((), dest_def, ret);
                }
            }
        } else {
            assert!(c_variadic || dest.is_none())
        }

        // para = arg ~> rho(para') + rho(arg') = rho(arg)
        for (para, arg) in callee_caller {
            if let Some(para) = para.clone() {
                if let Some(Consume {
                    r#use: arg_use,
                    def: arg_def,
                }) = arg.clone()
                {
                    for (para, arg_use, arg_def) in izip!(para, arg_use, arg_def) {
                        infer_cx
                            .database
                            .push_linear::<crate::analysis::constraint::Debug>(
                                (),
                                para,
                                arg_def,
                                arg_use,
                            );
                    }
                }
            } else {
                assert!(c_variadic || arg.is_none(), "{:?}", callee)
            }
        }
    }

    fn handle_inputs(
        inter_ctxt: &<WholeProgram as AnalysisKind>::InterCtxt,
        database: &mut DB,
        r#fn: DefId,
        inputs: impl Iterator<Item = Option<Range<OwnershipSig>>>,
    ) {
        let fn_sig = &inter_ctxt[&r#fn];

        for (input, sigs) in inputs.zip(fn_sig.iter().skip(1)) {
            // debug_assert!(!input.clone().xor(sigs.clone()).is_some())
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    for (input, sig) in input.zip(sigs.clone()) {
                        database.push_equal::<crate::analysis::constraint::Debug>((), input, sig)
                    }
                }
                (None, None) => {}
                _ => unreachable!(),
            }
        }
    }

    fn handle_output(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, WholeProgram>,
        r#fn: DefId,
        output: Option<Range<OwnershipSig>>,
    ) {
        // let r#fn = infer_cx.crate_ctxt.call_graph.did_idx[&r#fn];
        let fn_sig = &infer_cx.inter_ctxt[&r#fn];
        let ret = fn_sig.ret.clone();
        match (output, ret) {
            (Some(output), Some(ret)) => {
                for (output, ret) in output.zip(ret) {
                    infer_cx
                        .database
                        .push_equal::<crate::analysis::constraint::Debug>((), output, ret)
                }
            }
            (None, None) => {}
            _ => unreachable!(),
        }
    }
}
