use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;

use super::InferCtxt;
use crate::{
    call_graph::FnSig,
    ownership::{AnalysisKind, WholeProgram},
    ssa::{
        constraint::{Database, Var},
        consume::Consume,
    },
};

pub mod libc;
pub mod library;

/// Bad abstraction
/// TODO refactor
pub trait Boundary<'infercx, 'db, 'tcx>: AnalysisKind<'infercx, 'db> + Sized
where
    'tcx: 'infercx,
{
    fn handle_call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        caller: &FnSig<Option<Consume<Range<Var>>>>,
        callee: DefId,
    );

    fn handle_params(
        inter_ctxt: &Self::InterCtxt,
        database: &mut <Self as AnalysisKind<'infercx, 'db>>::DB,
        r#fn: DefId,
        params: impl Iterator<Item = Option<Range<Var>>>,
    );

    fn handle_ret(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        r#fn: DefId,
        ret: Option<Range<Var>>,
    );
}

impl<'infercx, 'db, 'tcx, Analysis> Boundary<'infercx, 'db, 'tcx> for Analysis
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    default fn handle_call(
        _: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        _: &FnSig<Option<Consume<Range<Var>>>>,
        _: DefId,
    ) {
    }

    default fn handle_params(
        _: &Analysis::InterCtxt,
        _: &mut <Self as AnalysisKind<'infercx, 'db>>::DB,
        _: DefId,
        _: impl Iterator<Item = Option<Range<Var>>>,
    ) {
    }

    default fn handle_ret(
        _: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        _: DefId,
        _: Option<Range<Var>>,
    ) {
    }
}

impl<'infercx, 'db, 'tcx> Boundary<'infercx, 'db, 'tcx> for WholeProgram
where
    'tcx: 'infercx,
{
    fn handle_call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        caller: &FnSig<Option<Consume<Range<Var>>>>,
        callee: DefId,
    ) {
        let c_variadic = infer_cx
            .crate_ctxt
            .tcx
            .fn_sig(callee)
            .skip_binder()
            .c_variadic;

        let callee_sigs = &infer_cx.inter_ctxt[&callee];

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
                        .push_assume::<crate::ssa::constraint::Debug>((), dest_use, false);
                    infer_cx
                        .database
                        .push_equal::<crate::ssa::constraint::Debug>((), dest_def, ret);
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
                            .push_linear::<crate::ssa::constraint::Debug>(
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

    fn handle_params(
        inter_ctxt: &<WholeProgram as AnalysisKind>::InterCtxt,
        database: &mut <WholeProgram as AnalysisKind>::DB,
        r#fn: DefId,
        params: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let fn_sig = &inter_ctxt[&r#fn];

        for (input, sigs) in params.zip(fn_sig.iter().skip(1)) {
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    for (input, sig) in input.zip(sigs.clone()) {
                        database.push_equal::<crate::ssa::constraint::Debug>((), input, sig)
                    }
                }
                (None, None) => {}
                _ => unreachable!(),
            }
        }
    }

    fn handle_ret(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, WholeProgram>,
        r#fn: DefId,
        ret: Option<Range<Var>>,
    ) {
        let fn_sig = &infer_cx.inter_ctxt[&r#fn];
        let sigs = fn_sig.ret.clone();
        match (ret, sigs) {
            (Some(output), Some(sigs)) => {
                for (output, ret) in output.zip(sigs) {
                    infer_cx
                        .database
                        .push_equal::<crate::ssa::constraint::Debug>((), output, ret)
                }
            }
            (None, None) => {}
            _ => unreachable!(),
        }
    }
}
