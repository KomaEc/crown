use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::Body;

use super::{matcher, CallArgs, InferCtxt};
use crate::{
    ownership::{whole_program::WholeProgramAnalysis, AnalysisKind},
    ssa::{
        constraint::{Database, Var},
        consume::Consume,
    },
};

pub mod libc;
pub mod library;

pub trait Boundary<'infercx, 'db, 'tcx>: AnalysisKind<'infercx, 'db> + Sized
where
    'tcx: 'infercx,
{
    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
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
        _: &CallArgs,
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

impl<'infercx, 'db, 'tcx> Boundary<'infercx, 'db, 'tcx> for WholeProgramAnalysis
where
    'tcx: 'infercx,
{
    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: DefId,
    ) {
        let fn_sig = infer_cx.fn_ctxt.tcx.fn_sig(callee);

        let mut params = infer_cx.inter_ctxt[&callee].iter();

        let ret = params.next().unwrap();

        // dest = ret ~> rho(dest) = 0, rho(dest') = rho(ret)
        if let Some(ret) = ret.clone() {
            if let Some(dest) = destination {
                let output_ty = fn_sig.output().skip_binder();

                let ret = ret.expect_normal();

                matcher(
                    output_ty,
                    dest.transpose(),
                    ret,
                    &infer_cx.fn_ctxt.struct_topology,
                    infer_cx.database,
                    |dest, ret, database| {
                        database.push_assume::<crate::ssa::constraint::Debug>(
                            (),
                            dest.r#use,
                            false,
                        );
                        database.push_equal::<crate::ssa::constraint::Debug>((), dest.def, ret);
                    },
                );
            }
        }

        let params_args = izip!(params, args, fn_sig.inputs().skip_binder()); // params.zip(args.iter());

        // para = arg ~> rho(para') + rho(arg') = rho(arg)
        for (param, arg, &ty) in params_args {
            if let Some(param) = param.clone() {
                if let Some((arg, is_ref)) = arg.clone() {
                    match param {
                        crate::ownership::Param::Output(output_param) => {
                            let mut output_param = output_param.transpose();
                            assert!(output_param.size_hint().1.unwrap() > 0);
                            if is_ref {
                                let _ = output_param.next().unwrap();
                            }
                            let arg = arg.transpose();

                            matcher(
                                ty,
                                output_param,
                                arg,
                                &infer_cx.fn_ctxt.struct_topology,
                                infer_cx.database,
                                |param, arg, database| {
                                    database.push_equal::<crate::ssa::constraint::Debug>(
                                        (),
                                        param.r#use,
                                        arg.r#use,
                                    );
                                    database.push_equal::<crate::ssa::constraint::Debug>(
                                        (),
                                        param.def,
                                        arg.def,
                                    );
                                },
                            );
                        }
                        crate::ownership::Param::Normal(param) => {
                            println!("param = {:?}, arg = {:?}", param, arg);

                            let arg = arg.transpose();

                            matcher(
                                ty,
                                param,
                                arg,
                                &infer_cx.fn_ctxt.struct_topology,
                                infer_cx.database,
                                |param, arg, database| {
                                    database
                                    .push_linear::<crate::ssa::constraint::Debug>(
                                        (),
                                        param,
                                        arg.def,
                                        arg.r#use,
                                    );
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    fn params(
        inter_ctxt: &<WholeProgramAnalysis as AnalysisKind>::InterCtxt,
        database: &mut <WholeProgramAnalysis as AnalysisKind>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let fn_sig = &inter_ctxt[&body.source.def_id()];

        for (input, sigs) in params.zip(fn_sig.iter().skip(1)) {
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    let sigs = sigs.clone().to_input();
                    assert_eq!(input.size_hint().1.unwrap(), sigs.size_hint().1.unwrap());
                    for (input, sig) in input.zip(sigs) {
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
            assert_eq!(arg.size_hint().1.unwrap(), param.size_hint().1.unwrap());
            for (arg, param) in arg.zip(param) {
                database.push_equal::<crate::ssa::constraint::Debug>((), arg, param);
            }
        }

        for (param, arg) in fn_sig.args.iter().cloned().zip(args) {
            if let Some((param, arg)) = param.zip(arg) {
                if let Some(param) = param.to_output() {
                    // if output then output
                    assert_eq!(arg.size_hint().1.unwrap(), param.size_hint().1.unwrap());
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
