use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::Body,
    ty::{Ty, TyCtxt, TyKind},
};

use super::{matcher, CallArgs, InferCtxt};
use crate::{
    ownership::{whole_program::WholeProgramAnalysis, AnalysisKind},
    ptr::Measurable,
    ssa::{
        constraint::{infer::InferMode, Database, GlobalAssumptions, Var},
        consume::Consume,
    },
    struct_topology::StructTopology,
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
        tcx: TyCtxt<'tcx>,
        inter_ctxt: &Self::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        struct_topology: &StructTopology,
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
        _: TyCtxt,
        _: &Analysis::InterCtxt,
        _: &GlobalAssumptions,
        _: &StructTopology,
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
                            let arg = arg.transpose();

                            matcher(
                                ty,
                                param,
                                arg,
                                &infer_cx.fn_ctxt.struct_topology,
                                infer_cx.database,
                                |param, arg, database| {
                                    database.push_linear::<crate::ssa::constraint::Debug>(
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
        tcx: TyCtxt<'tcx>,
        inter_ctxt: &<WholeProgramAnalysis as AnalysisKind>::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        struct_topology: &StructTopology,
        database: &mut <WholeProgramAnalysis as AnalysisKind>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let fn_sig = &inter_ctxt[&body.source.def_id()];

        for (input, sigs, ty) in itertools::izip!(
            params,
            fn_sig.iter().skip(1),
            body.args_iter().map(|local| body.local_decls[local].ty)
        ) {
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    let sigs = sigs.clone().to_input();
                    assert_eq!(input.size_hint().1.unwrap(), sigs.size_hint().1.unwrap());
                    let measure = input.size_hint().1.unwrap() as u32;
                    let precision = struct_topology.absolute_precision(ty, measure);

                    for (input, sig) in input.clone().zip(sigs) {
                        database.push_equal::<crate::ssa::constraint::Debug>((), input, sig)
                    }

                    let mut input = input;

                    apply_global_assumptions(
                        ty,
                        None,
                        &mut std::iter::empty(),
                        &mut input,
                        global_assumptions,
                        struct_topology,
                        database,
                        tcx,
                        precision,
                    );
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

impl<'infercx, 'db, 'tcx, Analysis> InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    fn unknown_call(&mut self, destination: Option<Consume<Range<Var>>>, args: &CallArgs) {
        if let Some(dest) = destination {
            <Analysis as InferMode>::borrow(self, dest);
        }
        for arg in args {
            if let Some((arg, _)) = arg.clone() {
                <Analysis as InferMode>::lend(self, arg);
            }
        }
    }
}

fn apply_global_assumptions<'tcx>(
    ty: Ty<'tcx>,
    mut dom: Option<Var>,
    field_ctxt: &mut dyn Iterator<Item = Var>,
    input: &mut impl Iterator<Item = Var>,
    global_assumptions: &GlobalAssumptions,
    struct_topology: &StructTopology,
    database: &mut impl Database,
    tcx: TyCtxt<'tcx>,
    mut precision: u8,
) {
    if precision == 0 {
        return;
    }

    let mut ty = ty;
    loop {
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }
        if let Some(ty_mut) = ty.builtin_deref(true) {
            let input = input.next().unwrap();
            if let Some((field, dom)) = field_ctxt.next().zip(dom) {
                database.push_eq_min::<crate::ssa::constraint::Debug>((), input, field, dom);
            }
            dom = Some(input);
            precision -= 1;
            if precision == 0 {
                return;
            }
            ty = ty_mut.ty;
            continue;
        }
        break;
    }

    if let TyKind::Adt(adt_def, subst) = ty.kind() {
        assert!(field_ctxt.next().is_none());
        if struct_topology.is_struct_of_concerned(&adt_def.did())
            && struct_topology.measure_adt(*adt_def, 0) > 0
        {
            let fields = global_assumptions.fields(struct_topology, &adt_def.did());
            for (mut field_ctxt, field_def) in itertools::izip!(fields, adt_def.all_fields()) {
                let field_ty = field_def.ty(tcx, subst);
                apply_global_assumptions(
                    field_ty,
                    dom,
                    &mut field_ctxt,
                    input,
                    global_assumptions,
                    struct_topology,
                    database,
                    tcx,
                    precision,
                )
            }
        }
    }
}
