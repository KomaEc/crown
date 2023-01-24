use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::Body,
    ty::{Ty, TyCtxt, TyKind},
};

use super::{matcher, CallArgs, InferCtxt};
use crate::{
    call_graph::Monotonicity,
    lattice::FlatSet,
    ownership::{whole_program::WholeProgramAnalysis, AnalysisKind},
    ptr::Measurable,
    ssa::{
        constraint::{infer::InferMode, Database, GlobalAssumptions, Var},
        consume::Consume,
    },
    struct_ctxt::StructCtxt,
    CrateCtxt,
};

pub mod libc;
pub mod library;

pub trait Boundary<'infercx, 'db, 'tcx>: AnalysisKind<'infercx, 'db, 'tcx> + Sized
where
    'tcx: 'infercx,
{
    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: DefId,
    );

    fn entry(
        crate_ctxt: &CrateCtxt<'tcx>,
        inter_ctxt: &Self::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        database: &mut <Self as AnalysisKind<'infercx, 'db, 'tcx>>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    );

    fn exit(
        tcx: TyCtxt<'tcx>,
        inter_ctxt: &Self::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        struct_ctxt: &StructCtxt,
        database: &mut Self::DB,
        body: &Body<'tcx>,
        args: impl Iterator<Item = Option<Range<Var>>>,
    );
}

impl<'infercx, 'db, 'tcx, Analysis> Boundary<'infercx, 'db, 'tcx> for Analysis
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
{
    default fn call(
        _: &mut InferCtxt<'infercx, 'db, 'tcx, Self>,
        _: Option<Consume<Range<Var>>>,
        _: &CallArgs,
        _: DefId,
    ) {
    }

    default fn entry(
        _: &CrateCtxt<'tcx>,
        _: &Analysis::InterCtxt,
        _: &GlobalAssumptions,
        _: &mut <Self as AnalysisKind<'infercx, 'db, 'tcx>>::DB,
        _: &Body<'tcx>,
        _: impl Iterator<Item = Option<Range<Var>>>,
    ) {
    }

    default fn exit(
        _: TyCtxt<'tcx>,
        _: &Self::InterCtxt,
        _: &GlobalAssumptions,
        _: &StructCtxt,
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
        let fn_sig = infer_cx.tcx.fn_sig(callee);

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
                    &infer_cx.struct_ctxt.unrestricted,
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
                            let ty = if is_ref {
                                let _ = output_param.next().unwrap();
                                ty.builtin_deref(true).unwrap().ty
                            } else {
                                ty
                            };
                            let arg = arg.transpose();

                            matcher(
                                ty,
                                output_param,
                                arg,
                                &infer_cx.struct_ctxt.unrestricted,
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
                            let mut param = param;
                            let ty = if is_ref {
                                tracing::debug!(
                                    "bad output parameter analysis for {}!",
                                    infer_cx.tcx.def_path_str(callee)
                                );
                                let _ = param.next().unwrap();
                                ty.builtin_deref(true).unwrap().ty
                            } else {
                                ty
                            };

                            // FIXME working around type cast
                            let mut arg = arg;
                            if let Some(pointee_ty) = ty.builtin_deref(true) {
                                let pointee_ty = pointee_ty.ty;
                                if format!("{pointee_ty}").ends_with("c_void") {
                                    arg.r#use = arg.r#use.start..arg.r#use.start + 1u32;
                                    arg.def = arg.def.start..arg.def.start + 1u32;
                                }
                            }
                            let arg = arg;

                            let arg = arg.transpose();

                            matcher(
                                ty,
                                param,
                                arg,
                                &infer_cx.struct_ctxt.unrestricted,
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

    fn entry(
        crate_ctxt: &CrateCtxt<'tcx>,
        inter_ctxt: &<WholeProgramAnalysis as AnalysisKind>::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        database: &mut <WholeProgramAnalysis as AnalysisKind>::DB,
        body: &Body<'tcx>,
        params: impl Iterator<Item = Option<Range<Var>>>,
    ) {
        let CrateCtxt {
            tcx,
            ref fn_ctxt,
            ref struct_ctxt,
        } = *crate_ctxt;
        let fn_sig = &inter_ctxt[&body.source.def_id()];

        for (input, sigs, ty) in itertools::izip!(
            params,
            fn_sig.iter().skip(1),
            body.args_iter().map(|local| body.local_decls[local].ty)
        ) {
            match (input, sigs) {
                (Some(input), Some(sigs)) => {
                    let is_output = sigs.is_output();
                    let input_sigs = sigs.clone().to_input();
                    assert_eq!(
                        input.size_hint().1.unwrap(),
                        input_sigs.size_hint().1.unwrap()
                    );
                    let measure = input.size_hint().1.unwrap() as u32;
                    let precision = struct_ctxt.absolute_precision(ty, measure);

                    for (input, sig) in input.clone().zip(input_sigs.clone()) {
                        database.push_equal::<crate::ssa::constraint::Debug>((), input, sig)
                    }

                    if !is_output {
                        let mut input = input;

                        apply_global_assumptions(
                            ty,
                            None,
                            &mut std::iter::empty(),
                            &mut input,
                            global_assumptions,
                            struct_ctxt,
                            database,
                            tcx,
                            precision,
                        );
                    } else {
                        let monotonicity = fn_ctxt.monotonicity(body.source.def_id());
                        let mut input_sigs = input_sigs;
                        let mut output_sigs = sigs.clone().to_output().unwrap();

                        if !matches!(monotonicity, FlatSet::Bottom) {
                            if !matches!(monotonicity, FlatSet::Elem(Monotonicity::Dealloc)) {
                                apply_global_assumptions(
                                    ty,
                                    None,
                                    &mut std::iter::empty(),
                                    &mut output_sigs,
                                    global_assumptions,
                                    struct_ctxt,
                                    database,
                                    tcx,
                                    precision,
                                );
                            }

                            if !matches!(monotonicity, FlatSet::Elem(Monotonicity::Alloc)) {
                                apply_global_assumptions(
                                    ty,
                                    None,
                                    &mut std::iter::empty(),
                                    &mut input_sigs,
                                    global_assumptions,
                                    struct_ctxt,
                                    database,
                                    tcx,
                                    precision,
                                );
                            }
                        }
                    }
                }
                (None, None) => {}
                _ => unreachable!(),
            }
        }
    }

    fn exit(
        tcx: TyCtxt<'tcx>,
        inter_ctxt: &Self::InterCtxt,
        global_assumptions: &GlobalAssumptions,
        struct_ctxt: &StructCtxt,
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
            for (arg, param) in arg.zip(param.clone()) {
                database.push_equal::<crate::ssa::constraint::Debug>((), arg, param);
            }

            let mut param = param;
            let ty = body.return_ty();
            let measure = param.size_hint().1.unwrap() as u32;
            let precision = struct_ctxt.absolute_precision(ty, measure);

            apply_global_assumptions(
                ty,
                None,
                &mut std::iter::empty(),
                &mut param,
                global_assumptions,
                struct_ctxt,
                database,
                tcx,
                precision,
            )
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
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
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
    struct_ctxt: &StructCtxt,
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
        if struct_ctxt.is_struct_of_concerned(&adt_def.did())
            && struct_ctxt.measure_adt(*adt_def, 0) > 0
        {
            let fields = global_assumptions.fields(struct_ctxt, &adt_def.did());
            for (mut field_ctxt, field_def) in itertools::izip!(fields, adt_def.all_fields()) {
                let field_ty = field_def.ty(tcx, subst);
                apply_global_assumptions(
                    field_ty,
                    dom,
                    &mut field_ctxt,
                    input,
                    global_assumptions,
                    struct_ctxt,
                    database,
                    tcx,
                    precision,
                )
            }
        }
    }
}
