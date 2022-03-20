pub mod boundary_model;
pub mod libcall_model;

use std::ops::Range;

use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        BasicBlock, Body, CastKind, Local, Location, Operand, Place, PlaceElem, PlaceRef,
        ProjectionElem, Rvalue, Statement, Terminator, TerminatorKind,
    },
    ty::{TyCtxt, TyKind::FnDef},
};
use rustc_target::abi::VariantIdx;

use crate::{
    boundary_model::BoundaryModel,
    call_graph::{CallGraph, CallSite, Func},
    def_use::IsDefUse,
    def_use::OwnershipAnalysisDefUse,
    libcall_model::LibCallModel,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{
            HasSSANameHandler, HasSSARenameState, SSAIdx, SSANameHandler, SSARename, SSARenameState,
        },
    },
    ty_ext::TyExt,
    Analysis, BoundaryE, CrateAnalysisCtxt, CrateAnalysisCtxtIntraView,
};

use super::{AnalysisEngine, ConstraintDataBase, Rho, FuncSummary, MaybeSaturated};

impl<'tcx> AnalysisEngine<'tcx> {
    pub fn infer<Handler: SSANameHandler<Output = ()>>(&mut self, mut extra_handlers: Handler) {
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<<Self as Analysis>::DefUse>(self.tcx);

            // let lambda_ctxt_start = self.rho_ctxt.assumptions.next_index();
            // let constraints_start = self.constraints.len();

            let infer_ctxt = InferCtxt::new(
                self.tcx,
                func,
                body,
                &self.call_graph,
                &mut self.rho_ctxt,
                self.globals.clone(),
                insertion_points.filter_repack(|local, _| {
                    body.local_decls[local]
                        .ty
                        .is_ptr_but_not_fn_ptr()
                        .then(|| vec![])
                }),
            );

            let mut infer: InferEngine<_> = InferEngine {
                ctxt: infer_ctxt,
                ssa_state: SSARenameState::new(&body.local_decls),
                extra_handlers: &mut extra_handlers,
            };

            infer.rename_body(body, &insertion_points);

            // self.func_summaries.push(FuncSummary { constraint_db: constraint_db, func_sig: val })
        }
    }
}

pub struct InferEngine<'infercx, 'tcx: 'infercx, Handler: SSANameHandler> {
    ctxt: InferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    // boundary_constraints: &'infercx mut IndexVec<CallSite, BoundaryE<()>>,
    // return_ssa_idx: Vec<SSAIdx>,
    extra_handlers: Handler,
}

impl<'infercx, 'tcx, Handler: SSANameHandler> HasSSARenameState<Local>
    for InferEngine<'infercx, 'tcx, Handler>
{
    #[inline]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSANameHandler
    for InferEngine<'infercx, 'tcx, Handler>
{
    type Output = Option<Range<Rho>>;

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

impl<'infercx, 'tcx, Handler: SSANameHandler> HasSSANameHandler
    for InferEngine<'infercx, 'tcx, Handler>
{
    type Handler = Self;

    type DefUse = OwnershipAnalysisDefUse;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSARename<'tcx>
    for InferEngine<'infercx, 'tcx, Handler>
{
    fn define_local(&mut self, local: Local, location: Location) -> Option<Range<Rho>> {
        if self.ctxt.body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
            Some(self.process_ptr_local(local, location).1)
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

enum PlaceProcessResult {
    Base { old: Range<Rho>, new: Range<Rho> },
    Proj(Range<Rho>),
}

impl<'infercx, 'tcx, Handler: SSANameHandler> InferEngine<'infercx, 'tcx, Handler> {
    #[inline]
    fn handle_ptr_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Range<Rho> {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.handle_def(local, idx, location)
    }

    #[inline]
    fn handle_ptr_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Range<Rho> {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.handle_use(local, idx, location)
    }

    fn process_ptr_local(&mut self, local: Local, location: Location) -> (Range<Rho>, Range<Rho>) {
        let old_ssa_idx = self.ssa_state().r#use(local);
        let old = self
            .ssa_name_handler()
            .handle_ptr_use(local, old_ssa_idx, location);
        let new_ssa_idx = self.ssa_state().define(local);
        let new = self
            .ssa_name_handler()
            .handle_ptr_def(local, new_ssa_idx, location);

        for (old_inner, new_inner) in std::iter::zip(old.clone().skip(1), new.clone().skip(1)) {
            self.ctxt
                .constraints
                .push_eq(old_inner, new_inner);
        }
        (old, new)
    }

    fn process_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> PlaceProcessResult {
        // this is a define
        if place.projection.is_empty() {
            let local = place.local;
            let (old, new) = self.process_ptr_local(local, location);

            return PlaceProcessResult::Base { old, new };
        }
        let ssa_idx = self.ssa_state().r#use(place.local);
        let base = self
            .ssa_name_handler()
            .handle_ptr_use(place.local, ssa_idx, location);
        PlaceProcessResult::Proj(self.proj_place_rhos(base, place.iter_projections()))
    }

    fn proj_place_rhos(
        &self,
        base: Range<Rho>,
        projections: impl Iterator<Item = (PlaceRef<'tcx>, PlaceElem<'tcx>)> + DoubleEndedIterator,
    ) -> Range<Rho> {
        let mut n_derefs = 0;
        for (base, proj) in projections.rev() {
            match proj {
                ProjectionElem::Field(field, _) => {
                    let place_ty = base.ty(self.ctxt.body, self.ctxt.tcx);
                    let ty = place_ty.ty;
                    let variant_idx = place_ty.variant_index.unwrap_or(VariantIdx::new(0));
                    let adt_def = ty.ty_adt_def().unwrap();
                    let rhos = self.ctxt.rho_ctxt.field_defs[&adt_def.did][variant_idx]
                        [field.index()]
                    .clone();
                    return Range {
                        start: rhos.start + n_derefs,
                        end: rhos.end,
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
}

impl<'infercx, 'tcx, Handler: SSANameHandler> Visitor<'tcx>
    for InferEngine<'infercx, 'tcx, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = <Self as HasSSANameHandler>::DefUse::categorize(context) {
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
                match (
                    self.process_ptr_place(place, location),
                    self.process_ptr_place(rhs, location),
                ) {
                    (
                        PlaceProcessResult::Base {
                            old: lhs_old,
                            new: lhs_new,
                        },
                        PlaceProcessResult::Base {
                            old: rhs_old,
                            new: rhs_new,
                        },
                    ) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(lhs_new.clone().skip(1), rhs_new.clone().skip(1))
                        {
                            self.ctxt
                                .constraints
                                .push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt
                            .constraints
                            .push_transfer(rhs_old.start, lhs_new.start, rhs_new.start);
                            // .push_eq(rhs_old.start, [lhs_new.start, rhs_new.start].into_iter());
                        self.ctxt.rho_ctxt.assume(lhs_old.start, false)
                    }
                    (PlaceProcessResult::Base { old, new }, PlaceProcessResult::Proj(f)) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(new.clone().skip(1), f.clone().skip(1))
                        {
                            self.ctxt
                                .constraints
                                .push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt
                            .constraints
                            .push_le(new.start, f.start);
                            // .push_ge(f.start, [new.start].into_iter());
                        self.ctxt.rho_ctxt.assume(old.start, false);
                    }
                    (PlaceProcessResult::Proj(f), PlaceProcessResult::Base { old, new }) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(f.clone().skip(1), new.clone().skip(1))
                        {
                            self.ctxt
                                .constraints
                                .push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt
                            .constraints
                            .push_transfer(old.start, f.start, new.start)
                            // .push_eq(old.start, [f.start, new.start].into_iter());
                    }
                    (PlaceProcessResult::Proj(f), PlaceProcessResult::Proj(g)) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(f.clone().skip(1), g.clone().skip(1))
                        {
                            self.ctxt
                                .constraints
                                .push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt
                            .constraints
                            .push_le(f.start, g.start);
                            // .push_ge(g.start, [f.start].into_iter());
                    }
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
                match callee_did.as_local() {
                    Some(did) => {
                        if matches!(
                            self.ctxt.tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::ForeignItem(_))
                        ) {
                            self.model_libc_call(callee_did, args, destination, location);
                            return;
                        }

                        unimplemented!("inter analysis")
                    }
                    None => {
                        self.model_library_call(callee_did, args, destination, location);
                        return;
                    }
                }
            }
        } else if let TerminatorKind::Return = terminator.kind {
            self.model_return(location);
            return;
        }
        self.super_terminator(terminator, location)
    }
}

pub struct InferCtxt<'infercx, 'tcx: 'infercx> {
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    rho_ctxt: CrateAnalysisCtxtIntraView<'infercx, Rho, Option<bool>>,
    phi_joins: PhiNodeInsertionPoints<Vec<SSAIdx>>,
    constraints: ConstraintDataBase<MaybeSaturated>, //Vec<Constraint>,
}

impl<'infercx, 'tcx> InferCtxt<'infercx, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        func: Func,
        body: &'infercx Body<'tcx>,
        call_graph: &'infercx CallGraph,
        rho_ctxt: &'infercx mut CrateAnalysisCtxt<Rho, Option<bool>>,
        globals: Range<Rho>,
        // constraints: &'infercx mut ConstraintDataBase, //Vec<Constraint>,
        phi_joins: PhiNodeInsertionPoints<Vec<SSAIdx>>,
    ) -> Self {

        let initial_locals_start = rho_ctxt.assumptions.next_index();
        let rho_ctxt = rho_ctxt.intra_view(body, func);
        let initial_locals_end = rho_ctxt.assumptions.next_index();

        let locals = Range { start: initial_locals_start, end: initial_locals_end };

        InferCtxt {
            tcx,
            body,
            call_graph,
            rho_ctxt,
            phi_joins,
            constraints: ConstraintDataBase::new(globals, locals),
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
            for (local, rhos) in self.rho_ctxt.locals.iter_enumerated() {
                assert_eq!(rhos.len(), 1);
                let rhos = &rhos[0usize.into()];
                for (nested_level, rho) in rhos.clone().enumerate() {
                    log::debug!(
                        "{}{:*<2$}{3:?}^0 ==> {4:?}",
                        INDENT,
                        "",
                        nested_level,
                        local,
                        rho
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
                for (local, rhos) in locals.iter_enumerated() {
                    assert!(rhos.len() >= 3);
                    log::debug!("for {:?} at {:?}, {:?}", local, bb, rhos)
                }
            }
        }
        self
    }
}

impl<'infercx, 'tcx> SSANameHandler for InferCtxt<'infercx, 'tcx> {
    type Output = Range<Rho>;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        log::debug!("InferCtxt defining {:?}^{} of ptr type", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        let res = self.rho_ctxt.generate_local(local, idx);

        // TODO: refactor this!
        for rho in res.start..res.end {
            self.constraints.le_constraints.add_local(rho)
        }


        res
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        self.rho_ctxt.locals[local][idx].clone()
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        self.rho_ctxt.generate_local(local, idx);
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
