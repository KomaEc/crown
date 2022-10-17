use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::Body;

use super::InferCtxt;
use crate::{
    ownership::{AnalysisKind, WholeProgram},
    ssa::{
        constraint::{infer::InferMode, Database, Var},
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
    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        destination: Option<Consume<Range<Var>>>,
        args: &<Self as InferMode<'infercx, 'db, 'tcx>>::CallArgs,
        // args: &FnSig<Option<Consume<Range<Var>>>>,
        callee: DefId,
    );

    fn params(
        inter_ctxt: &Self::InterCtxt,
        database: &mut <Self as AnalysisKind<'infercx, 'db>>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    );

    fn r#return(
        inter_ctxt: &Self::InterCtxt,
        database: &mut Self::DB,
        body: &Body<'tcx>,
        args: impl Iterator<Item = Option<Range<Var>>>,
    );
}

impl<'infercx, 'db, 'tcx, Analysis> Boundary<'infercx, 'db, 'tcx> for Analysis
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    default fn call(
        _: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        _: Option<Consume<Range<Var>>>,
        _: &<Self as InferMode<'infercx, 'db, 'tcx>>::CallArgs,
        _: DefId,
    ) {
    }

    default fn params(
        _: &Analysis::InterCtxt,
        _: &mut <Self as AnalysisKind<'infercx, 'db>>::DB,
        _: &Body<'tcx>,
        _: impl Iterator<Item = Option<Range<Var>>>,
    ) {
    }

    default fn r#return(
        _: &Self::InterCtxt,
        _: &mut Self::DB,
        _: &Body<'tcx>,
        _: impl Iterator<Item = Option<Range<Var>>>,
    ) {
    }
}

impl<'infercx, 'db, 'tcx> Boundary<'infercx, 'db, 'tcx> for WholeProgram
where
    'tcx: 'infercx,
{
    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        destination: Option<Consume<Range<Var>>>,
        args: &<Self as InferMode<'infercx, 'db, 'tcx>>::CallArgs,
        callee: DefId,
    ) {
        let c_variadic = infer_cx
            .crate_ctxt
            .tcx
            .fn_sig(callee)
            .skip_binder()
            .c_variadic;

        let mut params = infer_cx.inter_ctxt[&callee].iter();

        let ret = params.next().unwrap();

        // dest = ret ~> rho(dest) = 0, rho(dest') = rho(ret)
        if let Some(ret) = ret.clone() {
            if let Some(Consume {
                r#use: dest_use,
                def: dest_def,
            }) = destination
            {
                let ret = ret.expect_normal();
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
            assert!(destination.is_none())
        }

        let params_args = params.zip(args.iter());

        // para = arg ~> rho(para') + rho(arg') = rho(arg)
        for (param, arg) in params_args {
            if let Some(param) = param.clone() {
                if let Some((
                    Consume {
                        r#use: arg_use,
                        def: arg_def,
                    },
                    is_ref,
                )) = arg.clone()
                {
                    match param {
                        crate::ownership::Param::Output(output_param) => {
                            let Consume {
                                r#use: mut param_use,
                                def: mut param_def,
                            } = output_param;
                            if is_ref {
                                param_use.start += 1;
                                param_def.start += 1;
                            }
                            for (param_use, param_def, arg_use, arg_def) in
                                izip!(param_use, param_def, arg_use, arg_def)
                            {
                                infer_cx
                                    .database
                                    .push_equal::<crate::ssa::constraint::Debug>(
                                        (),
                                        param_use,
                                        arg_use,
                                    );
                                infer_cx
                                    .database
                                    .push_equal::<crate::ssa::constraint::Debug>(
                                        (),
                                        param_def,
                                        arg_def,
                                    );
                            }
                        }
                        crate::ownership::Param::Normal(param) => {
                            for (param, arg_use, arg_def) in izip!(param, arg_use, arg_def) {
                                infer_cx
                                    .database
                                    .push_linear::<crate::ssa::constraint::Debug>(
                                        (),
                                        param,
                                        arg_def,
                                        arg_use,
                                    );
                            }
                        }
                    }
                }
            } else {
                assert!(c_variadic || arg.is_none(), "{:?}", callee)
            }
        }
    }

    fn params(
        inter_ctxt: &<WholeProgram as AnalysisKind>::InterCtxt,
        database: &mut <WholeProgram as AnalysisKind>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let fn_sig = &inter_ctxt[&body.source.def_id()];

        for (input, sigs) in params.zip(fn_sig.iter().skip(1)) {
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    for (input, sig) in input.zip(sigs.clone().to_input()) {
                        database.push_equal::<crate::ssa::constraint::Debug>((), input, sig)
                    }
                }
                (None, None) => {}
                _ => unreachable!(),
            }
        }
    }

    fn r#return(
        inter_ctxt: &Self::InterCtxt,
        database: &mut Self::DB,
        body: &Body<'tcx>,
        mut args: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let fn_sig = &inter_ctxt[&body.source.def_id()];

        let ret_arg = args.next().unwrap();
        let ret_param = fn_sig.ret.clone();

        if let Some((arg, param)) = ret_arg.zip(ret_param) {
            let param = param.expect_normal();
            for (arg, param) in arg.zip(param) {
                database.push_equal::<crate::ssa::constraint::Debug>((), arg, param);
            }
        }

        for (param, arg) in fn_sig.args.iter().cloned().zip(args) {
            if let Some((param, arg)) = param.zip(arg) {
                if let Some(param) = param.to_output() {
                    // if output then output
                    for (arg, param) in arg.zip(param) {
                        database.push_equal::<crate::ssa::constraint::Debug>((), arg, param);
                    }
                } else {
                    // if not then finalize
                    for arg in arg {
                        database.push_assume::<crate::ssa::constraint::Debug>((), arg, false);
                    }
                }
            }
        }
    }
}
