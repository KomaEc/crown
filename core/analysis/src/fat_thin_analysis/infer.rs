pub mod libcall_model;

use std::ops::Range;

use crate::{
    call_graph::{CallGraph, CallSite, Func},
    def_use::{FatThinAnalysisDefUse, IsDefUse},
    fat_thin_analysis::{
        BoundaryConstraint, Constraint, ConstraintSet, CrateLambdaCtxt, CrateSummary, FuncSummary,
        Lambda, LambdaDataMap, LambdaSourceData,
    },
    libcall_model::LibCallModel,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{
            handler::SSANameSourceMap, HasSSANameHandler, HasSSARenameState, SSAIdx,
            SSANameHandler, SSARename, SSARenameState,
        },
    },
    ty_ext::TyExt,
    Analysis,
};
use graph::implementation::forward_star::Direction;
use range_ext::RangeExt;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Body, CastKind, Local, Location, Operand, Place, PlaceElem, PlaceRef,
        ProjectionElem, Rvalue, Statement, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::{subst::GenericArgKind, TyCtxt, TyKind::FnDef},
};
use rustc_target::abi::VariantIdx;

impl<'tcx> CrateSummary<'tcx> {
    pub fn infer_all<Handler: SSANameHandler<Output = ()>>(
        mut self,
        mut extra_handler: Handler,
    ) -> Self {
        let mut boundary_constraints = IndexVec::from_elem(vec![], &self.call_graph.graph.edges);
        let mut all_return_ssa_idx = IndexVec::with_capacity(self.call_graph.functions.len());
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<<Self as Analysis>::DefUse>(self.tcx);

            let mut ssa_name_source_map = SSANameSourceMap::new(body, &insertion_points);

            let lambda_ctxt_start = self.lambda_ctxt.lambda_data_map.len();
            let constraints_start = self.constraints.len();

            let mut infer: Infer<_> = Infer {
                ctxt: InferCtxt::new(
                    self.tcx,
                    func,
                    body,
                    &self.call_graph,
                    &mut self.lambda_ctxt,
                    &mut self.constraints,
                    insertion_points.filter_repack(|local, _| {
                        body.local_decls[local]
                            .ty
                            .is_ptr_but_not_fn_ptr()
                            .then(|| vec![])
                    }),
                ),
                boundary_constraints: &mut boundary_constraints,
                return_ssa_idx: vec![],
                ssa_state: SSARenameState::new(&body.local_decls),
                extra_handlers: (&mut extra_handler, &mut ssa_name_source_map),
            };

            infer.rename_body(body, &insertion_points);

            let InferCtxt {
                lambda_ctxt,
                phi_joins,
                ..
            } = infer.ctxt.log_phi_joins();

            let func_ctxt = lambda_ctxt.locals;
            let mut return_ssa_idx = infer.return_ssa_idx;
            self.lambda_ctxt.locals.push(func_ctxt);

            return_ssa_idx.sort();
            return_ssa_idx.dedup();
            log::debug!("process return places");
            let return_lambda = return_ssa_idx
                .split_first()
                .and_then(|(&this, rest)| {
                    let this = self.lambda_ctxt.locals[func][RETURN_PLACE][this].clone();// .row(this);
                    // although Return may occur multiple times (according to the docs), I'm
                    // curious to see how it may happen
                    debug_assert!(rest.is_empty(), "although Return may occur multiple times (according to the docs), I'm curious to see how it may happen");
                    for &other in rest {
                        let other = self.lambda_ctxt.locals[func][RETURN_PLACE][other].clone(); //.row(other);
                        for (this, other) in std::iter::zip(this.clone(), other) {
                            self.constraints.push_eq(this, other)
                        }
                    }
                    Some(this)
                })
                .unwrap_or_else(|| RangeExt::empty());

            assert_eq!(func, all_return_ssa_idx.push(return_ssa_idx));

            log::debug!("process equalities in phi nodes");
            for equalities in phi_joins.into_iter() {
                for (local, ssa_idxs) in equalities.into_iter_enumerated() {
                    let (&this_ssa_idx, rest_ssa_idxs) = ssa_idxs.split_first().unwrap();
                    let this = self.lambda_ctxt.locals[func][local][this_ssa_idx].clone();

                    for &other_ssa_idx in rest_ssa_idxs {
                        let other = self.lambda_ctxt.locals[func][local][other_ssa_idx].clone();
                        for (this, other) in std::iter::zip(this.clone(), other) {
                            self.constraints.push_eq(this, other);
                        }
                    }
                }
            }

            self.ssa_name_source_map.push(ssa_name_source_map);

            let lambda_ctxt_end = self.lambda_ctxt.lambda_data_map.len();
            let constraints_end = self.constraints.len();

            assert_eq!(
                func,
                self.func_summaries.push(FuncSummary {
                    lambda_ctxt: Range {
                        start: lambda_ctxt_start,
                        end: lambda_ctxt_end
                    },
                    constraints: Range {
                        start: constraints_start,
                        end: constraints_end
                    },
                    func_sig: std::iter::once(return_lambda)
                        .chain(body.args_iter().map(|local| {
                            body.local_decls[local]
                                .ty
                                .is_ptr_but_not_fn_ptr()
                                .then(|| {
                                    self.lambda_ctxt.locals[func][local][0usize.into()].clone()
                                }) // self.lambda_ctxt.locals[func][local].row(0).to_vec())
                                .unwrap_or_else(|| RangeExt::empty())
                        }))
                        .collect::<Vec<_>>(),
                })
            )
        }

        log::debug!("process boundary constraints");
        self.setup_boundary_constraints(boundary_constraints, &all_return_ssa_idx);

        self
    }

    pub fn setup_boundary_constraints(
        &mut self,
        boundary_constraints: IndexVec<CallSite, Vec<BoundaryConstraint>>,
        return_ssa_idx: &IndexVec<Func, Vec<SSAIdx>>,
    ) {
        self.boundary_constraints = boundary_constraints
            .into_iter_enumerated()
            .map(|(call_site, bcs)| {
                let mut res = vec![];
                let edge_data = &self.call_graph.graph.edges[call_site];
                log::debug!("Post processing boundary constraints");
                for bc in bcs {
                    match bc {
                        // callee = caller
                        BoundaryConstraint::Argument { caller, callee } => {
                            let callee = self.lambda_ctxt.locals[edge_data.target][callee]
                                [0usize.into()]
                            .clone(); //.row(0);
                            for (caller, callee) in std::iter::zip(caller, callee) {
                                res.push(Constraint(callee, caller))
                            }
                        }
                        // caller = callee
                        BoundaryConstraint::Return { caller, callee } => {
                            assert_eq!(callee, RETURN_PLACE);
                            let &callee_ssa_idx = return_ssa_idx[edge_data.target].first().unwrap();
                            let callee = self.lambda_ctxt.locals[edge_data.target][RETURN_PLACE]
                                [callee_ssa_idx]
                                .clone();
                            // .row(callee_ssa_idx);
                            for (caller, callee) in std::iter::zip(caller, callee) {
                                res.push(Constraint(caller, callee))
                            }
                        }
                    }
                }
                res
            })
            .collect::<IndexVec<_, _>>();
    }
}

impl CrateLambdaCtxt {
    pub fn intra_view(&mut self, body: &Body, func: Func) -> CrateLambdaCtxtIntraView<'_> {
        let locals = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                //let entry_fact =
                let start = self.lambda_data_map.next_index();
                local_decl
                    .ty
                    .walk()
                    .filter_map(|generic_arg| {
                        if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                            Some(ty)
                        } else {
                            None
                        }
                    })
                    .take_while(|ty| ty.is_ptr_but_not_fn_ptr())
                    .enumerate()
                    .for_each(|(nested_level, _)| {
                        self.lambda_data_map.push(
                            None,
                            LambdaSourceData::Local {
                                func,
                                base: local,
                                ssa_idx: 0usize.into(),
                                nested_level,
                            },
                        );
                    });
                let end = self.lambda_data_map.next_index();
                // .collect::<SmallVec<_>>();
                IndexVec::from_raw(vec![Range { start, end }])
            })
            .collect::<IndexVec<_, _>>();

        CrateLambdaCtxtIntraView {
            func,
            lambda_map: &mut self.lambda_data_map,
            field_defs: &self.field_defs,
            locals,
        }
    }
}

pub struct Infer<'infercx, 'tcx: 'infercx, Handler: SSANameHandler> {
    ctxt: InferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundary_constraints: &'infercx mut IndexVec<CallSite, Vec<BoundaryConstraint>>,
    return_ssa_idx: Vec<SSAIdx>,
    extra_handlers: Handler,
}

impl<'infercx, 'tcx, Handler: SSANameHandler> HasSSARenameState<Local>
    for Infer<'infercx, 'tcx, Handler>
{
    #[inline]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSANameHandler for Infer<'infercx, 'tcx, Handler> {
    type Output = Option<Range<Lambda>>;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_def(local, idx, location))
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_use(local, idx, location))
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_def_at_phi_node(local, idx, block));
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_use_at_phi_node(local, idx, block, pos));
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> HasSSANameHandler for Infer<'infercx, 'tcx, Handler> {
    type Handler = Self;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSARename<'tcx> for Infer<'infercx, 'tcx, Handler> {
    type DefUse = FatThinAnalysisDefUse;

    fn define_local(&mut self, local: Local, location: Location) -> Option<Range<Lambda>> {
        if self.ctxt.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
            Some(self.define_ptr_local(local, location))
        } else {
            let ssa_idx = self.ssa_state().define(local);
            // we don't call ctxt handler here because we already know that it is not a pointer
            self.extra_handlers.handle_def(local, ssa_idx, location);
            None
        }
    }

    #[inline]
    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.visit_statement(statement, location)
    }

    #[inline]
    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.visit_terminator(terminator, location)
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> Infer<'infercx, 'tcx, Handler> {
    #[inline]
    fn handle_ptr_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Range<Lambda> {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.handle_def(local, idx, location)
    }

    #[inline]
    fn handle_ptr_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Range<Lambda> {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.handle_use(local, idx, location)
    }

    fn define_ptr_local(&mut self, local: Local, location: Location) -> Range<Lambda> {
        // let old = self.ssa_state().r#use(local);
        // let old_range
        let old_range = self.use_ptr_local(local, location);
        let new = self.ssa_state().define(local);
        let new_range = self.handle_ptr_def(local, new, location);
        // Explicitly call inner handlers to avoid an additional ptr type test
        // self.handle_ptr_def(local, new, location);
        // let (&mut old_range, &mut new_range) = self.ctxt.lambda_ctxt.locals[local].pick2_mut(old, new);
        for (old, new) in std::iter::zip(
            old_range,         // self.ctxt.lambda_ctxt.locals[local][old],
            new_range.clone(), // self.ctxt.lambda_ctxt.locals[local][new],
        ) {
            self.ctxt.constraints.push_le(new, old)
        }
        new_range
    }

    #[inline]
    fn use_ptr_local(&mut self, local: Local, location: Location) -> Range<Lambda> {
        let ssa_idx = self.ssa_state().r#use(local);
        self.handle_ptr_use(local, ssa_idx, location)
    }

    fn proj_place_lambdas(
        &self,
        base: Range<Lambda>,
        projections: impl Iterator<Item = (PlaceRef<'tcx>, PlaceElem<'tcx>)> + DoubleEndedIterator,
    ) -> Range<Lambda> {
        let mut n_derefs = 0;
        for (base, proj) in projections.rev() {
            match proj {
                ProjectionElem::Field(field, _) => {
                    let place_ty = base.ty(self.ctxt.body, self.ctxt.tcx);
                    let ty = place_ty.ty;
                    let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                    let adt_def = ty.ty_adt_def().unwrap();
                    let lambdas = self.ctxt.lambda_ctxt.field_defs[&adt_def.did][variant_idx]
                        [field.index()]
                    .clone();
                    return Range {
                        start: lambdas.start + n_derefs,
                        end: lambdas.end,
                    };
                }
                ProjectionElem::Deref => n_derefs += 1,
                _ => unimplemented!("projections other than deref and field are not supported!"),
            }
        }
        let Range { start, end } = base;
        Range {
            start: start + n_derefs,
            end,
        }
    }

    fn use_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> Range<Lambda> {
        log::debug!("use ptr place {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        let ssa_idx = self.ssa_state().r#use(place.local);
        let base = self.handle_ptr_use(place.local, ssa_idx, location);
        self.proj_place_lambdas(base, place.iter_projections())
    }

    fn define_place_assume_simple_ptr(
        &mut self,
        place: &Place<'tcx>,
        location: Location,
    ) -> Lambda {
        let Range { start, end } = self.try_define_ptr_place(place, location);
        assert_eq!(start + 1, end);
        start
    }

    fn try_define_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> Range<Lambda> {
        log::debug!("try define ptr place {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        place
            .projection
            .is_empty()
            .then(|| self.define_ptr_local(place.local, location))
            .unwrap_or_else(|| self.use_ptr_place(place, location))
    }

    fn use_place_assume_simple_ptr(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        let Range { start, end } = self.use_ptr_place(place, location);
        assert_eq!(start + 1, end);
        start
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> Visitor<'tcx> for Infer<'infercx, 'tcx, Handler> {
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = <Self as SSARename>::DefUse::categorize(context) {
            if def_use.defining() {
                self.define_local(local, location);
            } else if def_use.using() {
                self.use_local(local, location);
            }
        }
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        use colored::*;
        log::debug!("{}", &format!("visiting statement {:?}", statement).bold()); //statement);
        self.super_statement(statement, location)
    }

    /// TODO: handle cases where `rvalue` is an `AddressOf`
    /// TODO: handle `CastKind::Pointer`? (this includes casting fat pointers to thin pointers)
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr()
        {
            if let Rvalue::Use(Operand::Move(rhs))
            | Rvalue::Use(Operand::Copy(rhs))
            | Rvalue::Cast(CastKind::Misc, Operand::Move(rhs), _)
            | Rvalue::Cast(CastKind::Misc, Operand::Copy(rhs), _) = rvalue
            {
                let lhs = self.try_define_ptr_place(place, location);
                let rhs = self.use_ptr_place(rhs, location);
                assert_eq!(lhs.len(), rhs.len());
                for (lhs, rhs) in std::iter::zip(lhs, rhs) {
                    self.ctxt.constraints.push_le(lhs, rhs);
                }
                return;
            }
        }
        use colored::*;
        log::warn!(
            "{}",
            "this statement is not processed by the intra inferencer"
                .bold()
                .red()
        );
        self.super_assign(place, rvalue, location)
    }

    /// TODO: extract initial summary from global context!
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        use colored::*;
        log::debug!(
            "{}",
            &format!("visiting terminator {:?}", terminator.kind).bold()
        );
        if let TerminatorKind::Call {
            ref func,
            ref args,
            destination,
            cleanup: _,
            from_hir_call: _,
            fn_span: _,
        } = terminator.kind
        {
            let ty = func
                .constant()
                .expect("closures or function pointers are not supported!")
                .ty();
            if let &FnDef(callee_did, _generic_args) = ty.kind() {
                // local defined functions: libc externs or user functions
                match callee_did.as_local() {
                    Some(did) => {
                        // let hir_id = self.ctxt.tcx.hir().local_def_id_to_hir_id(did);
                        if matches!(
                            self.ctxt.tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::ForeignItem(_))
                        ) {
                            self.model_libc_call(callee_did, args, destination, location);
                            return;
                        } else if matches!(
                            self.ctxt.tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::Item(_))
                        ) {
                            // Does it help the compiler with bound check elimination?
                            assert_eq!(
                                self.ctxt.call_graph.call_sites.len(),
                                self.ctxt.call_graph.graph.edges.len()
                            );
                            let (call_site, edge_data) = self
                                .ctxt
                                .call_graph
                                .graph
                                .adjacent_edges(self.ctxt.lambda_ctxt.func, Direction::Outgoing)
                                .find(|&(call_site, _)| {
                                    self.ctxt.call_graph.call_sites[call_site] == location
                                })
                                .unwrap();
                            debug_assert_eq!(edge_data.source, self.ctxt.lambda_ctxt.func);
                            debug_assert_eq!(
                                edge_data.target,
                                self.ctxt.call_graph.lookup_function(&callee_did).unwrap()
                            );
                            for (idx, arg) in args.iter().enumerate() {
                                if arg
                                    .ty(self.ctxt.body, self.ctxt.tcx)
                                    .is_ptr_but_not_fn_ptr()
                                {
                                    let place = arg
                                        .place()
                                        .expect("constant in call arguments is not supported");
                                    let lambdas = self.use_ptr_place(&place, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Argument {
                                            caller: lambdas,
                                            callee: Local::from_usize(idx + 1),
                                        },
                                    );
                                    log::debug!(
                                        "generate boundary constraint ({:?}, {:?}) ≤ ({:?}. {:?})",
                                        edge_data.target,
                                        Local::from_usize(idx + 1),
                                        edge_data.source,
                                        arg
                                    )
                                } else {
                                    self.visit_operand(arg, location)
                                }
                            }
                            if let Some((destination, _)) = destination {
                                if destination
                                    .ty(self.ctxt.body, self.ctxt.tcx)
                                    .ty
                                    .is_ptr_but_not_fn_ptr()
                                {
                                    let lambdas = self.try_define_ptr_place(&destination, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Return {
                                            caller: lambdas,
                                            callee: Place::return_place().local,
                                        },
                                    );
                                    log::debug!(
                                        "generate boundary constraint ({:?}, {:?}) ≤ ({:?}, {:?})",
                                        edge_data.source,
                                        destination,
                                        edge_data.target,
                                        Place::return_place().local,
                                    )
                                } else {
                                    self.visit_place(
                                        &destination,
                                        PlaceContext::MutatingUse(MutatingUseContext::Call),
                                        location,
                                    )
                                }
                            }
                            return;
                        }
                    }
                    // library functions
                    None => {
                        self.model_library_call(callee_did, args, destination, location);
                        return;
                    }
                }
            } else {
                unreachable!("what could it be? {}", ty)
            }
        } else if let TerminatorKind::Return = terminator.kind {
            if self.ctxt.body.local_decls[RETURN_PLACE]
                .ty
                .is_ptr_but_not_fn_ptr()
            {
                let ssa_idx = self.ssa_state().r#use(RETURN_PLACE);
                self.ssa_name_handler()
                    .handle_use(RETURN_PLACE, ssa_idx, location);
                self.return_ssa_idx.push(ssa_idx);
            }
            return;
        }
        self.super_terminator(terminator, location)
    }
}

pub struct CrateLambdaCtxtIntraView<'intracx> {
    pub func: Func,
    pub lambda_map: &'intracx mut LambdaDataMap<Option<bool>>,
    pub field_defs: &'intracx FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Lambda>>>>,
    pub locals: IndexVec<Local, IndexVec<SSAIdx, Range<Lambda>>>, // IndexVec<Local, IndexVec<SSAIdx, SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>,
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    /// return the `ssa_idx`
    pub fn generate_local(&mut self, base: Local, ssa_idx: SSAIdx) -> Range<Lambda> {
        let lambdas = &self.locals[base];
        let entry_fact = &lambdas[0usize.into()];
        let nested_level = entry_fact.len();
        assert!(nested_level > 0);

        let n_facts = lambdas.len();
        assert_eq!(n_facts, ssa_idx.index());
        // let new_fact = (0..nested_level)

        let start = self.lambda_map.next_index();
        (0..nested_level).for_each(|nested_level| {
            let lambda = self.lambda_map.push(
                None,
                LambdaSourceData::Local {
                    func: self.func,
                    base,
                    ssa_idx,
                    nested_level,
                },
            );
            log::debug!(
                "generate {:?} for {:*<2$}{base:?}^{ssa_idx}",
                lambda,
                "",
                nested_level
            );
            // lambda
        });
        let end = self.lambda_map.next_index();

        let lambdas = Range { start, end };
        // .collect::<SmallVec<_>>();
        self.locals[base].push(lambdas.clone());
        // n_facts

        lambdas
    }
}

pub struct InferCtxt<'infercx, 'tcx: 'infercx> {
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    phi_joins: PhiNodeInsertionPoints<Vec<SSAIdx>>,
    constraints: &'infercx mut ConstraintSet, //Vec<Constraint>,
}

impl<'infercx, 'tcx> InferCtxt<'infercx, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        func: Func,
        body: &'infercx Body<'tcx>,
        call_graph: &'infercx CallGraph,
        lambda_ctxt: &'infercx mut CrateLambdaCtxt,
        constraints: &'infercx mut ConstraintSet, //Vec<Constraint>,
        phi_joins: PhiNodeInsertionPoints<Vec<SSAIdx>>,
    ) -> Self {
        InferCtxt {
            tcx,
            body,
            call_graph,
            lambda_ctxt: lambda_ctxt.intra_view(body, func),
            phi_joins,
            constraints,
        }
        .log_initial_state()
    }

    fn log_initial_state(self) -> Self {
        #[cfg(debug_assertions)]
        {
            const INDENT: &str = "   ";
            log::debug!(
                "for function {}:",
                self.tcx.def_path_debug_str(self.body.source.def_id())
            );
            for (local, lambdas) in self.lambda_ctxt.locals.iter_enumerated() {
                assert_eq!(lambdas.len(), 1);
                let lambdas = &lambdas[0usize.into()];
                for (nested_level, lambda) in lambdas.clone().enumerate() {
                    log::debug!(
                        "{}{:*<2$}{3:?}^0 ==> {4:?}",
                        INDENT,
                        "",
                        nested_level,
                        local,
                        lambda
                    )
                }
            }
        }
        self
    }

    fn log_phi_joins(self) -> Self {
        #[cfg(debug_assertions)]
        {
            log::debug!("Phi nodes joins:");
            for (bb, locals) in self.phi_joins.iter_enumerated() {
                for (local, lambdas) in locals.iter_enumerated() {
                    assert!(lambdas.len() >= 3);
                    log::debug!("for {:?} at {:?}, {:?}", local, bb, lambdas)
                }
            }
        }
        self
    }
}

impl<'infercx, 'tcx> SSANameHandler for InferCtxt<'infercx, 'tcx> {
    type Output = Range<Lambda>;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        log::debug!("InferCtxt defining {:?}^{} of ptr type", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.lambda_ctxt.generate_local(local, idx)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        self.lambda_ctxt.locals[local][idx].clone()
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.lambda_ctxt.generate_local(local, idx);
        self.phi_joins[block][local].push(idx)
    }

    fn handle_use_at_phi_node(
        &mut self,
        local: Local,
        idx: SSAIdx,
        block: BasicBlock,
        _pos: usize,
    ) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.phi_joins[block][local].push(idx)
    }
}