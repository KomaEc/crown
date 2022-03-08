pub mod model_call;

use std::{marker::PhantomData, ops::Range};

use crate::{
    array_analysis::{
        BoundaryConstraint, Constraint, ConstraintSet, CrateLambdaCtxt, CrateSummary, FuncSummary,
        Lambda, LambdaMap, LambdaSourceData,
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
use ndarray::{Array2, ArrayView};
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

impl<'tcx, DefUse: IsDefUse> CrateSummary<'tcx, DefUse> {
    pub fn infer_all<Handler: SSANameHandler<Output = ()>>(
        mut self,
        mut extra_handler: Handler,
    ) -> Self {
        let mut boundary_constraints = IndexVec::from_elem(vec![], &self.call_graph.graph.edges);
        let mut all_return_ssa_idx = IndexVec::with_capacity(self.call_graph.functions.len());
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<DefUse>(self.tcx);

            let mut ssa_name_source_map = SSANameSourceMap::new(body, &insertion_points);

            let lambda_ctxt_start = self.lambda_ctxt.lambda_map.len();
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
                        let nested_level = body.local_decls[local]
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
                            .count();

                        (nested_level > 0)
                            .then(|| Array2::from_shape_vec((0, nested_level), vec![]).unwrap())
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
            let return_lambda = return_ssa_idx.split_first().map(|(&this, rest)| {
                let this = self.lambda_ctxt.locals[func][RETURN_PLACE].row(this);
                // although Return may occur multiple times (according to the docs), I'm
                // curious to see how it may happen
                assert!(rest.is_empty());
                for &other in rest {
                    let other = self.lambda_ctxt.locals[func][RETURN_PLACE].row(other);
                    for (&this, &other) in std::iter::zip(this, other) {
                        self.constraints.push_eq(this, other)
                    }
                }
                this.as_slice().unwrap()
            });
            let return_lambda = return_lambda
                .map(|slice| slice.to_vec())
                .unwrap_or_else(|| vec![]);

            assert_eq!(func, all_return_ssa_idx.push(return_ssa_idx));

            log::debug!("process equalities in phi nodes");
            for equalities in phi_joins.into_iter() {
                for equalities in equalities.into_iter() {
                    for equality in equalities.columns() {
                        let &this = equality.first().unwrap();
                        for &other in equality.iter().skip(1) {
                            self.constraints.push_eq(this, other);
                        }
                    }
                    /*
                    assert!(equality.len() >= 2);
                    let (&this, tail) = equality.split_first().unwrap();
                    for &other in tail {
                        self.constraints.push_eq(this, other);
                    }
                    */
                }
            }

            self.ssa_name_source_map.push(ssa_name_source_map);

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
                    },
                    func_sig: std::iter::once(return_lambda)
                        .chain(body.args_iter().map(|local| {
                            body.local_decls[local]
                                .ty
                                .is_ptr_but_not_fn_ptr()
                                .then(|| self.lambda_ctxt.locals[func][local].row(0).to_vec())
                                .unwrap_or_else(|| vec![])
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
        boundary_constraints: IndexVec<CallSite, Vec<BoundaryConstraint<'_, 'tcx>>>,
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
                                place,
                                ssa_idx,
                                body,
                                self.tcx,
                            );
                            let callee = self.lambda_ctxt.locals[edge_data.target][callee].row(0);
                            for (&caller, &callee) in std::iter::zip(caller.iter(), callee) {
                                res.push(Constraint(callee, caller))
                            }
                        }
                        // caller = callee
                        BoundaryConstraint::Return {
                            caller: (place, ssa_idx),
                            callee,
                        } => {
                            let caller = lookup_lambdas(
                                &self.lambda_ctxt.field_defs,
                                &self.lambda_ctxt.locals[func],
                                &place,
                                ssa_idx,
                                body,
                                self.tcx,
                            );
                            let &callee_ssa_idx = return_ssa_idx[edge_data.target].first().unwrap();
                            let callee = self.lambda_ctxt.locals[edge_data.target][RETURN_PLACE]
                                .row(callee_ssa_idx);
                            for (&caller, &callee) in std::iter::zip(caller.iter(), callee) {
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
        let local = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                let row = local_decl
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
                        self.lambda_map.push(
                            None,
                            LambdaSourceData::Local {
                                func,
                                base: local,
                                ssa_idx: 0,
                                nested_level,
                            },
                        )
                    })
                    .collect::<Vec<_>>();
                Array2::from_shape_vec((1, row.len()), row).unwrap()
            })
            .collect::<IndexVec<_, _>>();

        CrateLambdaCtxtIntraView {
            func,
            lambda_map: &mut self.lambda_map,
            field_defs: &self.field_defs,
            locals: local,
        }
    }
}

pub struct Infer<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>> {
    ctxt: InferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundary_constraints: &'infercx mut IndexVec<CallSite, Vec<BoundaryConstraint<'infercx, 'tcx>>>,
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
    type Output = usize;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_def(local, idx, location));
        idx
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.body.local_decls[local]
            .ty
            .is_ptr_but_not_fn_ptr()
            .then(|| self.ctxt.handle_use(local, idx, location));
        idx
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
    fn define(&mut self, local: Local, location: Location) -> usize {
        // FIXME: do not call extra handlers??
        let old = self.r#use(local, location);
        let ssa_idx = self.ssa_state().define(local);
        let new = self.ssa_name_handler().handle_def(local, ssa_idx, location);
        if self.ctxt.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
            for (&old, &new) in std::iter::zip(
                self.ctxt.lambda_ctxt.locals[local].row(old),
                self.ctxt.lambda_ctxt.locals[local].row(new),
            ) {
                self.ctxt.constraints.push_le(new, old)
            }
        }
        new
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
    /*
    /// FIXME: handle nested ptrs properly!
    /// Change the return type as an iterator over lambdas
    /// TODO!!!!
    /// !!!!!!!!!
    /// !!!!!!!!
    /// Move this logic into LambdaCtxt!!!
    /// request lambdas!!!
    fn process_projections(
        &self,
        // base: Local,
        place: Place<'tcx>,
        ssa_idx: usize,
        // projections: impl Iterator<Item = (PlaceRef<'tcx>, PlaceElem<'tcx>)> + DoubleEndedIterator,
    ) -> &[Lambda] {
        let mut iter = place.iter_projections().rev();
        let mut n_derefs = 0;
        while let Some((base, proj)) = iter.next() {
            match proj {
                ProjectionElem::Deref => {
                    n_derefs += 1;
                }
                ProjectionElem::Field(field, _) => {
                    let place_ty = base.ty(self.ctxt.body, self.ctxt.tcx);
                    let ty = place_ty.ty;
                    let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                    let adt_def = ty.ty_adt_def().unwrap();
                    return &self.ctxt.lambda_ctxt.field_defs[&adt_def.did][variant_idx]
                        [field.index()][n_derefs..];
                }
                _ => unimplemented!("projections other than deref and field are not supported!"),
            }
        }

        &self.ctxt.lambda_ctxt.local[place.local]
            .row(ssa_idx)
            .as_slice()
            .unwrap()[n_derefs..]
    }
    */

    fn pre_process_rhs(&mut self, place: &Place<'tcx>, location: Location) -> usize {
        log::debug!("processing rhs {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        let ssa_idx = self.r#use(place.local, location);

        ssa_idx
        //self.process_projections(place.local, ssa_idx, place.iter_projections())
    }

    fn pre_process_lhs(&mut self, place: &Place<'tcx>, location: Location) -> usize {
        log::debug!("processing lhs {:?}", place);

        debug_assert!(place
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr());

        let ssa_idx = place
            .projection
            .is_empty()
            .then(|| self.define(place.local, location))
            .unwrap_or_else(|| self.r#use(place.local, location));

        ssa_idx
        // self.process_projections(place.local, ssa_idx, place.iter_projections())
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
            .is_ptr_but_not_fn_ptr()
        {
            if let Rvalue::Use(Operand::Move(rhs))
            | Rvalue::Use(Operand::Copy(rhs))
            | Rvalue::Cast(CastKind::Misc, Operand::Move(rhs), _)
            | Rvalue::Cast(CastKind::Misc, Operand::Copy(rhs), _) = rvalue
            {
                let lhs_ssa_idx = self.pre_process_lhs(place, location);
                let rhs_ssa_idx = self.pre_process_rhs(rhs, location);
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
                                    let place_ssa_idx = self.pre_process_rhs(&place, location);
                                    self.boundary_constraints[call_site].push(
                                        BoundaryConstraint::Argument {
                                            caller: (arg, Some(place_ssa_idx)),
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
                                        self.pre_process_lhs(&destination, location);
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
    pub lambda_map: &'intracx mut LambdaMap<Option<bool>>,
    pub field_defs: &'intracx FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    pub locals: IndexVec<Local, Array2<Lambda>>,
}

#[inline]
pub fn lookup_lambdas<'a, 'tcx>(
    field_defs: &'a FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    locals: &'a IndexVec<Local, Array2<Lambda>>,
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

    &locals[place.local].row(ssa_idx).as_slice().unwrap()[n_derefs..]
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    /// return the `ssa_idx`
    pub fn generate_local(&mut self, base: Local, ssa_idx: usize) -> usize {
        // ArrayView1<Lambda> {
        let lambda_array_view = &self.locals[base];
        let ncols = lambda_array_view.ncols();
        assert!(ncols > 0);

        let nrows = lambda_array_view.nrows();
        assert_eq!(nrows, ssa_idx);
        let new_row = (0..ncols)
            .map(|nested_level| {
                self.lambda_map.push(
                    None,
                    LambdaSourceData::Local {
                        func: self.func,
                        base,
                        ssa_idx,
                        nested_level,
                    },
                )
            })
            .collect::<Vec<_>>();

        self.locals[base]
            .push_row(ArrayView::from(&new_row))
            .unwrap();
        // log::debug!("generate {:?} for Local {:?}^{}", lambda, base, ssa_idx);
        // self.local[base].row(nrow)
        nrows
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

pub struct InferCtxt<'infercx, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    /// [[_1^0, *_1^0, **_1^0],
    ///  [_1^3, *_1^3, **_1^3],
    ///  [_1^7, *_1^7, **_1^7]]
    phi_joins: PhiNodeInsertionPoints<Array2<Lambda>>,
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
        phi_joins: PhiNodeInsertionPoints<Array2<Lambda>>,
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
            for (local, y) in self.lambda_ctxt.locals.iter_enumerated() {
                if self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
                    todo!();
                    /*
                    assert_eq!(y.len(), 1);
                    log::debug!("{}{:?}^0 ==> {:?}", INDENT, local, y[0])
                    */
                } else {
                    assert!(y.is_empty())
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
    type Output = usize; // ArrayView1<'a, Lambda>;

    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("InferCtxt defining {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.lambda_ctxt.generate_local(local, idx)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        log::debug!("InferCtxt phi node defining {:?}^{}", local, idx);
        let lambda = self.lambda_ctxt.generate_local(local, idx);
        let lambdas = self.lambda_ctxt.locals[local].row(lambda);
        self.phi_joins[block][local].push_row(lambdas);
    }

    fn handle_use(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        log::debug!("InferCtxt using {:?}^{}", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        // let lambda = self.lambda_ctxt.local[local][[idx, 0]];
        // log::debug!("retrieve {:?} for Local {:?}^{}", lambda, local, idx);
        todo!()
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, _pos: usize) {
        log::debug!("InferCtxt phi node using {:?}^{}", local, idx);
        // let lambda = self.lambda_ctxt.local[local][[idx, 0]];
        let lambdas = self.lambda_ctxt.locals[local].row(idx);
        // log::debug!("retrieve {:?} for Local {:?}^{}", lambda, local, idx);
        self.phi_joins[block][local].push_row(lambdas);
    }
}
