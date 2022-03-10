pub mod model_call;

use std::{marker::PhantomData, ops::Range};

use crate::{
    array_analysis::{
        BoundaryConstraint, Constraint, ConstraintSet, CrateLambdaCtxt, CrateSummary, FuncSummary,
        Lambda, LambdaDataMap, LambdaSourceData,
    },
    call_graph::{CallGraph, CallSite, Func},
    def_use::IsDefUse,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{
            handler::SSANameSourceMap, HasSSANameHandler, HasSSARenameState, SSANameHandler,
            SSARename, SSARenameState,
        },
    },
    ty_ext::TyExt,
};
use graph::implementation::forward_star::Direction;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Body, CastKind, Local, Location, Operand, Place, ProjectionElem, Rvalue,
        Statement, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::{subst::GenericArgKind, TyCtxt, TyKind::FnDef},
};
use rustc_target::abi::VariantIdx;
use smallvec::{smallvec, SmallVec};

use super::NESTED_LEVEL_HINT;

impl<'tcx, DefUse: IsDefUse> CrateSummary<'tcx, DefUse> {
    pub fn infer_all<Handler: SSANameHandler>(mut self, mut extra_handler: Handler) -> Self {
        let mut boundary_constraints = IndexVec::from_elem(vec![], &self.call_graph.graph.edges);
        let mut all_return_ssa_idx = IndexVec::with_capacity(self.call_graph.functions.len());
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<DefUse>(self.tcx);

            let mut ssa_name_source_map = SSANameSourceMap::new(body, &insertion_points);

            let lambda_ctxt_start = self.lambda_ctxt.lambda_data_map.len();
            let constraints_start = self.constraints.len();

            let mut infer: Infer<DefUse, _> = Infer {
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
                _marker: PhantomData,
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
            // assert!(!return_ssa_idx.is_empty());
            let return_lambda = return_ssa_idx
                .split_first()
                .and_then(|(&this, rest)| {
                    let this = &self.lambda_ctxt.locals[func][RETURN_PLACE][this][..];// .row(this);
                    // although Return may occur multiple times (according to the docs), I'm
                    // curious to see how it may happen
                    debug_assert!(rest.is_empty(), "although Return may occur multiple times (according to the docs), I'm curious to see how it may happen");
                    for &other in rest {
                        let other = &self.lambda_ctxt.locals[func][RETURN_PLACE][other][..]; //.row(other);
                        for (&this, &other) in std::iter::zip(this, other) {
                            self.constraints.push_eq(this, other)
                        }
                    }
                    Some(this.into_iter().map(|&l| l).collect::<SmallVec<_>>())
                })
                .unwrap_or_else(|| smallvec![]);

            assert_eq!(func, all_return_ssa_idx.push(return_ssa_idx));

            log::debug!("process equalities in phi nodes");
            for equalities in phi_joins.into_iter() {
                for (local, ssa_idxs) in equalities.into_iter_enumerated() {
                    let (&this_ssa_idx, rest_ssa_idxs) = ssa_idxs.split_first().unwrap();
                    let this = &self.lambda_ctxt.locals[func][local][this_ssa_idx][..];

                    for &other_ssa_idx in rest_ssa_idxs {
                        let other = &self.lambda_ctxt.locals[func][local][other_ssa_idx][..];
                        for (&this, &other) in std::iter::zip(this, other) {
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
                                .then(|| self.lambda_ctxt.locals[func][local][0].clone()) // self.lambda_ctxt.locals[func][local].row(0).to_vec())
                                .unwrap_or_else(|| smallvec![])
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
        boundary_constraints: IndexVec<CallSite, Vec<BoundaryConstraint<'tcx>>>,
        return_ssa_idx: &IndexVec<Func, Vec<usize>>,
    ) {
        self.boundary_constraints = boundary_constraints
            .into_iter_enumerated()
            .map(|(call_site, bcs)| {
                let mut res = vec![];
                let edge_data = &self.call_graph.graph.edges[call_site];
                let func = edge_data.source;
                let body = self.call_graph.functions[func];
                let body = self.tcx.optimized_mir(body);
                log::debug!("Post processing boundary constraints");
                for bc in bcs {
                    match bc {
                        // callee = caller
                        BoundaryConstraint::Argument {
                            caller: (Operand::Copy(place), ssa_idx),
                            callee,
                        }
                        | BoundaryConstraint::Argument {
                            caller: (Operand::Move(place), ssa_idx),
                            callee,
                        } => {
                            let ssa_idx = ssa_idx.unwrap();
                            let caller = lookup_lambdas(
                                &self.lambda_ctxt.field_defs,
                                &self.lambda_ctxt.locals[func],
                                &place,
                                ssa_idx,
                                body,
                                self.tcx,
                            );
                            let callee = &self.lambda_ctxt.locals[edge_data.target][callee][0][..]; //.row(0);
                            for (&caller, &callee) in std::iter::zip(caller.iter(), callee) {
                                res.push(Constraint(callee, caller))
                            }
                        }
                        // caller = callee
                        BoundaryConstraint::Return {
                            caller: (place, ssa_idx),
                            callee,
                        } => {
                            assert_eq!(callee, RETURN_PLACE);
                            let caller = lookup_lambdas(
                                &self.lambda_ctxt.field_defs,
                                &self.lambda_ctxt.locals[func],
                                &place,
                                ssa_idx,
                                body,
                                self.tcx,
                            );
                            let &callee_ssa_idx = return_ssa_idx[edge_data.target].first().unwrap();
                            let callee = &self.lambda_ctxt.locals[edge_data.target][RETURN_PLACE]
                                [callee_ssa_idx][..];
                            // .row(callee_ssa_idx);
                            for (&caller, &callee) in std::iter::zip(caller.iter(), callee) {
                                res.push(Constraint(caller, callee))
                            }
                        }
                        _ => unimplemented!(),
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
                let entry_fact = local_decl
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
                    .map(|(nested_level, _)| {
                        self.lambda_data_map.push(
                            None,
                            LambdaSourceData::Local {
                                func,
                                base: local,
                                ssa_idx: 0,
                                nested_level,
                            },
                        )
                    })
                    .collect::<SmallVec<_>>();
                vec![entry_fact]
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

pub struct Infer<'infercx, 'tcx: 'infercx, DefUse: IsDefUse, Handler: SSANameHandler> {
    ctxt: InferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundary_constraints: &'infercx mut IndexVec<CallSite, Vec<BoundaryConstraint<'tcx>>>,
    return_ssa_idx: Vec<usize>,
    extra_handlers: Handler,
    _marker: PhantomData<*const DefUse>,
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler> HasSSARenameState<Local>
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler> SSANameHandler
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_def(local, idx, location));
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
        self.extra_handlers.handle_use(local, idx, location);
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_def_at_phi_node(local, idx, block));
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_use_at_phi_node(local, idx, block, pos));
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler> HasSSANameHandler
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Handler = Self;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler> SSARename<'tcx>
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type DefUse = DefUse;

    fn define_local(&mut self, local: Local, location: Location) -> usize {
        if self.ctxt.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
            self.define_ptr_local(local, location)
        } else {
            let ssa_idx = self.ssa_state().define(local);
            self.ssa_name_handler().handle_def(local, ssa_idx, location);
            ssa_idx
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

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn handle_ptr_def(&mut self, local: Local, idx: usize, location: Location) {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.handle_def(local, idx, location);
    }

    fn define_ptr_local(&mut self, local: Local, location: Location) -> usize {
        let old = self.ssa_state().r#use(local);
        let new = self.ssa_state().define(local);
        // Explicitly call inner handlers to avoid an additional ptr type test
        self.handle_ptr_def(local, new, location);
        for (&old, &new) in std::iter::zip(
            &self.ctxt.lambda_ctxt.locals[local][old],
            &self.ctxt.lambda_ctxt.locals[local][new],
        ) {
            self.ctxt.constraints.push_le(new, old)
        }
        new
    }

    fn use_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> usize {
        log::debug!("use ptr place {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        let ssa_idx = self.use_local(place.local, location);

        ssa_idx
    }

    fn define_place_assume_simple_ptr(
        &mut self,
        place: &Place<'tcx>,
        location: Location,
    ) -> Lambda {
        let ssa_idx = self.try_define_ptr_place(place, location);
        let lambdas =
            self.ctxt
                .lambda_ctxt
                .lookup_lambdas(place, ssa_idx, self.ctxt.body, self.ctxt.tcx);
        assert_eq!(lambdas.len(), 1);
        lambdas[0]
    }

    fn try_define_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> usize {
        log::debug!("try define ptr place {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        let ssa_idx = place
            .projection
            .is_empty()
            .then(|| self.define_ptr_local(place.local, location))
            .unwrap_or_else(|| self.use_local(place.local, location));

        ssa_idx
    }

    fn use_place_assume_simple_ptr(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        let ssa_idx = self.use_ptr_place(place, location);
        let lambdas =
            self.ctxt
                .lambda_ctxt
                .lookup_lambdas(place, ssa_idx, self.ctxt.body, self.ctxt.tcx);
        assert_eq!(lambdas.len(), 1);
        lambdas[0]
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler> Visitor<'tcx>
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorize(context) {
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
                let lhs_ssa_idx = self.try_define_ptr_place(place, location);
                let rhs_ssa_idx = self.use_ptr_place(rhs, location);
                let lhs = self.ctxt.lambda_ctxt.lookup_lambdas(
                    place,
                    lhs_ssa_idx,
                    self.ctxt.body,
                    self.ctxt.tcx,
                );
                let rhs = self.ctxt.lambda_ctxt.lookup_lambdas(
                    rhs,
                    rhs_ssa_idx,
                    self.ctxt.body,
                    self.ctxt.tcx,
                );
                assert_eq!(lhs.len(), rhs.len());
                for (&lhs, &rhs) in std::iter::zip(lhs.into_iter(), rhs.into_iter()) {
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
                                    let place_ssa_idx = self.use_ptr_place(&place, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Argument {
                                            caller: (arg.clone(), Some(place_ssa_idx)),
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
                                    let place_ssa_idx =
                                        self.try_define_ptr_place(&destination, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Return {
                                            caller: (destination, place_ssa_idx),
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
    pub field_defs: &'intracx FxHashMap<
        DefId,
        IndexVec<VariantIdx, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>,
    >,
    pub locals: IndexVec<Local, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>,
}

#[inline]
pub fn lookup_lambdas<'a, 'tcx>(
    field_defs: &'a FxHashMap<
        DefId,
        IndexVec<VariantIdx, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>,
    >,
    locals: &'a IndexVec<Local, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>,
    place: &Place<'tcx>,
    ssa_idx: usize,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> &'a [Lambda] {
    let mut iter = place.iter_projections().rev();
    let mut n_derefs = 0;
    while let Some((base, proj)) = iter.next() {
        match proj {
            ProjectionElem::Deref => {
                n_derefs += 1;
            }
            ProjectionElem::Field(field, _) => {
                let place_ty = base.ty(body, tcx);
                let ty = place_ty.ty;
                let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                let adt_def = ty.ty_adt_def().unwrap();
                return &field_defs[&adt_def.did][variant_idx][field.index()][n_derefs..];
            }
            _ => unimplemented!("projections other than deref and field are not supported!"),
        }
    }

    &locals[place.local][ssa_idx][n_derefs..]
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    /// return the `ssa_idx`
    pub fn generate_local(&mut self, base: Local, ssa_idx: usize) {
        let lambdas = &self.locals[base];
        let entry_fact = &lambdas[0];
        let nested_level = entry_fact.len();
        assert!(nested_level > 0);

        let n_facts = lambdas.len();
        assert_eq!(n_facts, ssa_idx);
        let new_fact = (0..nested_level)
            .map(|nested_level| {
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
                lambda
            })
            .collect::<SmallVec<_>>();
        self.locals[base].push(new_fact);
        // n_facts
    }

    pub fn lookup_lambdas<'tcx>(
        &self,
        place: &Place<'tcx>,
        ssa_idx: usize,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> &[Lambda] {
        lookup_lambdas(&self.field_defs, &self.locals, place, ssa_idx, body, tcx)
    }
}

pub struct InferCtxt<'infercx, 'tcx: 'infercx> {
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    phi_joins: PhiNodeInsertionPoints<Vec<usize>>,
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
        phi_joins: PhiNodeInsertionPoints<Vec<usize>>,
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
            const INDENT: &str = "   in f, ";
            log::debug!(
                "for function {}:",
                self.tcx.def_path_debug_str(self.body.source.def_id())
            );
            for (local, lambdas) in self.lambda_ctxt.locals.iter_enumerated() {
                assert_eq!(lambdas.len(), 1);
                let lambdas = &lambdas[0];
                for (nested_level, &lambda) in lambdas.iter().enumerate() {
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
    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) {
        log::debug!("InferCtxt defining {:?}^{} of ptr type", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.lambda_ctxt.generate_local(local, idx);
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        // log::debug!("InferCtxt phi node defining {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.lambda_ctxt.generate_local(local, idx);
        self.phi_joins[block][local].push(idx)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, _pos: usize) {
        // log::debug!("InferCtxt phi node using {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.phi_joins[block][local].push(idx)
    }
}
