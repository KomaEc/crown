pub mod model_call;

use std::{marker::PhantomData, ops::Range};

use crate::{
    array_analysis::{
        BoundaryConstraint, Constraint, ConstraintSet, CrateLambdaCtxt, CrateSummary,
        FuncLambdaCtxt, FuncSummary, Lambda, LambdaMap, LambdaSourceData,
    },
    call_graph::{CallGraph, CallSite, Func},
    def_use::IsDefUse,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{HasSSANameHandler, HasSSARenameState, SSANameHandler, SSARename, SSARenameState},
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
        BasicBlock, Body, CastKind, Local, Location, Operand, Place, PlaceElem, PlaceRef,
        ProjectionElem, Rvalue, Statement, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::{subst::GenericArgKind, TyCtxt, TyKind::FnDef},
};
use rustc_target::abi::VariantIdx;

impl<'tcx> CrateSummary<'tcx> {
    pub fn infer_all<DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>(
        mut self,
        extra_handler: &mut Handler,
    ) -> Self {
        let mut boundary_constraints = IndexVec::from_elem(vec![], &self.call_graph.graph.edges);
        let mut all_return_ssa_idx = IndexVec::with_capacity(self.call_graph.functions.len());
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<DefUse>(self.tcx);

            let lambda_ctxt_start = self.lambda_ctxt.lambda_map.len();
            let constraints_start = self.constraints.len();

            let mut infer: Infer<DefUse, &mut Handler> = Infer {
                ctxt: InferCtxt::new(
                    self.tcx,
                    func,
                    body,
                    &self.call_graph,
                    &mut self.lambda_ctxt,
                    &mut self.constraints,
                    insertion_points.filter_map_local(|local| {
                        body.local_decls[local]
                            .ty
                            .is_ptr_of_concerned()
                            .then(|| vec![])
                    }),
                ),
                boundary_constraints: &mut boundary_constraints,
                return_ssa_idx: vec![],
                ssa_state: SSARenameState::new(&body.local_decls),
                extra_handlers: &mut *extra_handler,
                _marker: PhantomData,
            };

            infer.rename_body(body, &insertion_points);

            #[cfg(debug_assertions)]
            infer.ctxt.debug_result();

            let InferCtxt {
                lambda_ctxt,
                phi_joins,
                ..
            } = infer.ctxt;

            let return_ssa_idx = infer.return_ssa_idx;

            let func_ctxt = FuncLambdaCtxt {
                local: lambda_ctxt.local,
                local_nested: lambda_ctxt.local_nested,
            };
            self.lambda_ctxt.func_ctxt.push(func_ctxt);
            assert_eq!(func, all_return_ssa_idx.push(return_ssa_idx));

            log::debug!("process equalities in phi nodes");
            for equalities in phi_joins.into_iter() {
                for equality in equalities.into_iter() {
                    assert!(equality.len() >= 2);
                    let (&this, tail) = equality.split_first().unwrap();
                    for &other in tail {
                        self.constraints.push_eq(this, other);
                    }
                }
            }

            let lambda_ctxt_end = self.lambda_ctxt.lambda_map.len();
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
                    }
                })
            )
        }

        log::debug!("process boundary constraints");
        self.setup_boundary_constraints(boundary_constraints, all_return_ssa_idx);

        self
    }

    pub fn setup_boundary_constraints(
        &mut self,
        boundary_constraints: IndexVec<CallSite, Vec<BoundaryConstraint>>,
        return_ssa_idx: IndexVec<Func, Vec<usize>>,
    ) {
        self.boundary_constraints = boundary_constraints
            .into_iter_enumerated()
            .map(|(call_site, bcs)| {
                let mut res = vec![];
                let edge_data = &self.call_graph.graph.edges[call_site];
                log::error!("The processing of boundary constraints is incomplete as this moment");
                for bc in bcs {
                    match bc {
                        // callee = caller
                        BoundaryConstraint::Argument { caller, callee } => {
                            assert!(
                                !self.lambda_ctxt.func_ctxt[edge_data.target].local[callee]
                                    .is_empty(),
                                "Argument {:?} should be initialised.",
                                callee
                            );
                            let &callee = self.lambda_ctxt.func_ctxt[edge_data.target].local
                                [callee]
                                .first()
                                .unwrap();
                            res.push(Constraint(callee, caller))
                        }
                        // caller = callee
                        BoundaryConstraint::Return { caller, callee } => {
                            assert!(!return_ssa_idx[edge_data.target].is_empty());
                            assert_eq!(callee, RETURN_PLACE);
                            for &ssa_idx in &return_ssa_idx[edge_data.target] {
                                let callee = self.lambda_ctxt.func_ctxt[edge_data.target].local
                                    [RETURN_PLACE][ssa_idx];
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
        let (local, local_nested) = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                let ty = local_decl.ty;
                (
                    // vec![],
                    ty.is_ptr_of_concerned()
                        .then(|| {
                            let lambda = self.lambda_map.push(
                                None,
                                LambdaSourceData::LocalScalar {
                                    func,
                                    base: local,
                                    ssa_idx: 0,
                                },
                            );
                            vec![lambda]
                        })
                        .unwrap_or_else(|| vec![]),
                    ty.walk()
                        .filter_map(|generic_arg| {
                            if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .take_while(|ty| ty.is_ptr_of_concerned())
                        .skip(1)
                        .enumerate()
                        .map(|(level, _)| {
                            self.lambda_map.push(
                                None,
                                LambdaSourceData::LocalNested {
                                    func,
                                    base: local,
                                    nested_level: level,
                                },
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();

        CrateLambdaCtxtIntraView {
            func,
            lambda_map: &mut self.lambda_map,
            field_defs: &self.field_defs,
            local,
            local_nested,
        }
    }
}

pub struct Infer<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> {
    ctxt: InferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundary_constraints: &'infercx mut IndexVec<CallSite, Vec<BoundaryConstraint>>,
    return_ssa_idx: Vec<usize>,
    extra_handlers: Handler,
    _marker: PhantomData<*const DefUse>,
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    HasSSARenameState<Local> for Infer<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> SSANameHandler
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Output = Option<Lambda>;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_of_concerned()
            .then(|| self.ctxt.handle_def(local, idx, location))
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_of_concerned()
            .then(|| self.ctxt.handle_use(local, idx, location))
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_of_concerned()
            .then(|| self.ctxt.handle_def_at_phi_node(local, idx, block));
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_of_concerned()
            .then(|| self.ctxt.handle_use_at_phi_node(local, idx, block, pos));
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> HasSSANameHandler
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Handler = Self;
    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> SSARename<'tcx>
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type DefUse = DefUse;

    /// Overide `define` so that the engine will first use a local before re-defining it
    fn define(&mut self, local: Local, location: Location) -> Option<Lambda> {
        // FIXME: do not call extra handlers??
        let old = self.r#use(local, location);
        let ssa_idx = self.ssa_state().define(local);
        let new = self
            .ssa_name_handler()
            .handle_def(local, ssa_idx, location)?;
        if let Some(old) = old {
            /*
            let constraint = Constraint(new, old);
            log::debug!(
                "generate constraint {} because of re-definition",
                constraint
            );
            self.ctxt.constraints.push(constraint);
            */
            self.ctxt.constraints.push_le(new, old)
        }
        Some(new)
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

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    /// FIXME: handle nested ptrs properly!
    /// Change the return type as an iterator over lambdas
    fn process_projections(
        &mut self,
        base: Local,
        length: usize,
        projections: impl Iterator<Item = (PlaceRef<'tcx>, PlaceElem<'tcx>)> + DoubleEndedIterator,
    ) -> Lambda {
        projections
            .rev()
            .enumerate()
            .find_map(|(nested_level, (base, proj))| {
                log::debug!("visiting projection {:?}", proj);
                if let ProjectionElem::Field(field, _) = proj {
                    let place_ty = base.ty(self.ctxt.body, self.ctxt.tcx);
                    let ty = place_ty.ty;
                    let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                    let adt_def = ty.ty_adt_def().unwrap();
                    Some(
                        self.ctxt.lambda_ctxt.field_defs[&adt_def.did][variant_idx][field.index()]
                            [nested_level],
                    )
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.ctxt.lambda_ctxt.local_nested[base][length - 1])
    }

    fn process_rhs(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        log::debug!("processing rhs {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_of_concerned());

        let lambda = self.r#use(place.local, location);

        if place.projection.is_empty() {
            return lambda.unwrap();
        }

        self.process_projections(
            place.local,
            place.projection.len(),
            place.iter_projections(),
        )
    }

    fn process_lhs(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        log::debug!("processing lhs {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_of_concerned());

        if place.projection.is_empty() {
            self.define(place.local, location).unwrap()
        } else {
            self.process_projections(
                place.local,
                place.projection.len(),
                place.iter_projections(),
            )
        }
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> Visitor<'tcx>
    for Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorize(context) {
            if def_use.defining() {
                self.define(local, location);
            } else if def_use.using() {
                self.r#use(local, location);
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
            .is_ptr_of_concerned()
        {
            if let Rvalue::Use(Operand::Move(rhs))
            | Rvalue::Use(Operand::Copy(rhs))
            | Rvalue::Cast(CastKind::Misc, Operand::Move(rhs), _)
            | Rvalue::Cast(CastKind::Misc, Operand::Copy(rhs), _) = rvalue
            {
                let lhs = self.process_lhs(place, location);
                let rhs = self.process_rhs(rhs, location);
                self.ctxt.constraints.push_le(lhs, rhs);
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
                                if arg.ty(self.ctxt.body, self.ctxt.tcx).is_ptr_of_concerned() {
                                    let place = arg
                                        .place()
                                        .expect("constant in call arguments is not supported");
                                    let lambda = self.process_rhs(&place, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Argument {
                                            caller: lambda,
                                            callee: Local::from_usize(idx + 1),
                                        },
                                    );
                                    log::debug!(
                                        "generate boundary constraint ({:?}, {:?}) ≤ {:?}",
                                        edge_data.target,
                                        Local::from_usize(idx + 1),
                                        lambda
                                    )
                                } else {
                                    self.visit_operand(arg, location)
                                }
                            }
                            if let Some((destination, _)) = destination {
                                if destination
                                    .ty(self.ctxt.body, self.ctxt.tcx)
                                    .ty
                                    .is_ptr_of_concerned()
                                {
                                    let lambda = self.process_lhs(&destination, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Return {
                                            caller: lambda,
                                            callee: Place::return_place().local,
                                        },
                                    );
                                    log::debug!(
                                        "generate boundary constraint {:?} ≤ ({:?}, {:?})",
                                        lambda,
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
            let ssa_idx = self.ssa_state().r#use(RETURN_PLACE);
            self.ssa_name_handler()
                .handle_use(RETURN_PLACE, ssa_idx, location);
            self.return_ssa_idx.push(ssa_idx);
            return;
        }
        self.super_terminator(terminator, location)
    }
}

pub struct CrateLambdaCtxtIntraView<'intracx> {
    pub func: Func,
    pub lambda_map: &'intracx mut LambdaMap<Option<bool>>,
    pub field_defs: &'intracx FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    pub local: IndexVec<Local, Vec<Lambda>>,
    pub local_nested: IndexVec<Local, Vec<Lambda>>,
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    pub fn generate_local(&mut self, base: Local, ssa_idx: usize) -> Lambda {
        let lambda = self.lambda_map.push(
            None,
            LambdaSourceData::LocalScalar {
                func: self.func,
                base,
                ssa_idx,
            },
        );
        self.local[base].push(lambda);
        log::debug!("generate {:?} for Local {:?}^{}", lambda, base, ssa_idx);
        lambda
    }
}

pub struct InferCtxt<'infercx, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    phi_joins: PhiNodeInsertionPoints<Vec<Lambda>>,
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
        phi_joins: PhiNodeInsertionPoints<Vec<Lambda>>,
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
            for (local, x) in self.lambda_ctxt.local_nested.iter_enumerated() {
                for (idx, lambda) in x.iter().enumerate() {
                    log::debug!(
                        "{}{:*<2$}{3:?} ==> {4:?}",
                        INDENT,
                        "",
                        idx + 1,
                        local,
                        lambda
                    )
                }
            }
            for (local, y) in self.lambda_ctxt.local.iter_enumerated() {
                if self.body.local_decls[local].ty.is_ptr_of_concerned() {
                    assert_eq!(y.len(), 1);
                    log::debug!("{}{:?}^0 ==> {:?}", INDENT, local, y[0])
                } else {
                    assert!(y.is_empty())
                }
            }
        }
        self
    }

    #[cfg(debug_assertions)]
    pub fn debug_result(&self) {
        log::debug!("Phi nodes joins:");
        for (bb, locals) in self.phi_joins.iter_enumerated() {
            for (local, lambdas) in locals.iter_enumerated() {
                log::debug!("for {:?} at {:?}, {:?}", local, bb, lambdas)
            }
        }
    }
}

impl<'infercx, 'tcx> SSANameHandler for InferCtxt<'infercx, 'tcx> {
    type Output = Lambda;

    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("InferCtxt defining {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_of_concerned());
        self.lambda_ctxt.generate_local(local, idx)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        log::debug!("InferCtxt phi node defining {:?}^{}", local, idx);
        let lambda = self.lambda_ctxt.generate_local(local, idx);
        self.phi_joins[block][local].push(lambda);
    }

    fn handle_use(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("InferCtxt using {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_of_concerned());
        let lambda = self.lambda_ctxt.local[local][idx];
        log::debug!("retrieve {:?} for Local {:?}^{}", lambda, local, idx);
        lambda
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, _pos: usize) {
        log::debug!("InferCtxt phi node using {:?}^{}", local, idx);
        let lambda = self.lambda_ctxt.local[local][idx];
        log::debug!("retrieve {:?} for Local {:?}^{}", lambda, local, idx);
        self.phi_joins[block][local].push(lambda);
    }
}
