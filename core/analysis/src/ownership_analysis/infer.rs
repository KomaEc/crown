pub mod boundary_model;
pub mod libcall_model;

use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        BasicBlock, Body, CastKind, Local, Location, Operand, Place, PlaceElem, PlaceRef,
        ProjectionElem, Rvalue, Statement, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::{adjustment::PointerCast, subst::GenericArgKind, TyCtxt, TyKind::FnDef},
};
use rustc_target::abi::VariantIdx;

use smallvec::smallvec;

use crate::{
    boundary_model::BoundaryModel,
    call_graph::CallGraph,
    def_use::IsDefUse,
    def_use::OwnershipAnalysisDefUse,
    libcall_model::LibCallModel,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{
            handler::{SSADefSites, SSANameSourceMap},
            HasSSANameHandler, HasSSARenameState, SSAIdx, SSANameHandler, SSARename,
            SSARenameState,
        },
    },
    ty_ext::TyExt,
    FuncSig, Surface,
};

use super::{
    AnalysisEngine, ConstraintDatabase, IntraSummary, LocalSourceInfo, OwnershipAnalysisBoundary,
    Rho,
};

use range_ext::RangeExt;

impl<'analysis, 'tcx> AnalysisEngine<'analysis, 'tcx> {
    pub fn infer<Handler: SSANameHandler<Output = ()>>(&mut self, mut extra_handlers: Handler) {
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            log::debug!("Inferring {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<OwnershipAnalysisDefUse>(self.tcx);

            let infer_ctxt = IntraInferCtxt::new(
                self.tcx,
                // func,
                body,
                &self.call_graph,
                self.inter_ctxt.global_assumptions.clone(),
                &self.inter_ctxt.field_defs,
                insertion_points.filter_repack(|local, _| {
                    body.local_decls[local]
                        .ty
                        .is_ptr_but_not_fn_ptr()
                        .then(|| None)
                }),
            );

            let mut infer: IntraInfer<_> = IntraInfer {
                ctxt: infer_ctxt,
                ssa_state: SSARenameState::new(&body.local_decls),
                boundaries: Vec::with_capacity(
                    self.call_graph
                        .graph
                        .adjacent_edges(
                            func,
                            graph::implementation::forward_star::Direction::Outgoing,
                        )
                        .size_hint()
                        .0,
                ),
                extra_handlers: (
                    &mut extra_handlers,
                    SSANameSourceMap::new(body, &insertion_points),
                    SSADefSites::new(body),
                ),
            };

            infer.rename_body(body, &insertion_points);

            self.inter_ctxt
                .global_assumptions
                .join_global_facts(&infer.ctxt.constraint_system, None);

            let (func_sig, intra_summary) = infer.complete(|h| (h.1, h.2));

            assert_eq!(self.intra_summaries.push(intra_summary), func);
            assert_eq!(self.func_sigs.push(func_sig), func);
        }
    }
}

pub struct IntraInfer<'infercx, 'tcx: 'infercx, Handler: SSANameHandler> {
    ctxt: IntraInferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundaries: Vec<OwnershipAnalysisBoundary>,
    extra_handlers: Handler,
}

impl<'infercx, 'tcx, Handler: SSANameHandler> HasSSARenameState<Local>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    #[inline]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSANameHandler
    for IntraInfer<'infercx, 'tcx, Handler>
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
    for IntraInfer<'infercx, 'tcx, Handler>
{
    type Handler = Self;

    type DefUse = OwnershipAnalysisDefUse;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSARename<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
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

#[derive(Clone)]
pub enum PtrPlaceDefResult {
    Base { old: Range<Rho>, new: Range<Rho> },
    Proj(Range<Rho>),
}

impl<'infercx, 'tcx, Handler: SSANameHandler> IntraInfer<'infercx, 'tcx, Handler> {
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
            self.ctxt.constraint_system.push_eq(old_inner, new_inner);
        }
        (old, new)
    }

    fn process_ptr_place(&mut self, place: &Place<'tcx>, location: Location) -> PtrPlaceDefResult {
        // this is a define
        if place.projection.is_empty() {
            let local = place.local;
            let (old, new) = self.process_ptr_local(local, location);

            return PtrPlaceDefResult::Base { old, new };
        }
        let ssa_idx = self.ssa_state().r#use(place.local);
        let base = self
            .ssa_name_handler()
            .handle_ptr_use(place.local, ssa_idx, location);
        PtrPlaceDefResult::Proj(self.proj_place_rhos(base, place.iter_projections()))
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
                    let rhos = self.ctxt.field_defs[&adt_def
                        .did
                        .as_local()
                        .expect("struct definitions should be in scope!!!")][variant_idx]
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

    fn complete(
        self,
        f: impl FnOnce(
            Handler,
        ) -> (
            SSANameSourceMap<OwnershipAnalysisDefUse>,
            SSADefSites<OwnershipAnalysisDefUse>,
        ),
    ) -> (FuncSig<Surface, Option<bool>>, IntraSummary) {
        let IntraInfer {
            ctxt,
            boundaries,
            extra_handlers,
            ..
        } = self;

        let (ssa_name_source_map, ssa_def_sites) = f(extra_handlers);

        let IntraInferCtxt {
            body,
            intra_source_map,
            locals,
            constraint_system,
            return_vars_rep,
            ..
        } = ctxt;

        // let mut r#abstract = Vec::new();

        let ret_rhos = return_vars_rep.unwrap_or_else(|| {
            /*
            assert_eq!(locals[RETURN_PLACE].len(), 1);
            let empty_range = locals[RETURN_PLACE][SSAIdx::ENTRY].clone();
            assert_eq!(empty_range.len(), 0);
            empty_range
            */
            assert!(locals[RETURN_PLACE].is_empty());
            RangeExt::empty()
        });

        let mut surface = vec![smallvec![None; ret_rhos.len()]];
        let mut inner = vec![ret_rhos];

        for arg in locals.iter().skip(1).take(body.arg_count) {
            if !arg.is_empty() {
                let rhos = arg[SSAIdx::ENTRY].clone();
                surface.push(smallvec![None; rhos.len()]);
                inner.push(rhos);
            } else {
                surface.push(smallvec![]);
                inner.push(RangeExt::empty());
            }
        }

        log::debug!("Generating surface function signature {:?}", &surface);
        log::debug!("Generating function signature {:?}", &inner);

        (
            FuncSig { sig: surface },
            IntraSummary {
                constraint_system,
                locals,
                intra_source_map,
                boundaries,
                ssa_def_sites,
                ssa_name_source_map,
                fn_sig: FuncSig { sig: inner },
            },
        )
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> Visitor<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = <Self as HasSSANameHandler>::DefUse::categorize_finely(
            local,
            &self.ctxt.body.local_decls,
            context,
        ) {
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
            | Rvalue::Cast(CastKind::Misc, Operand::Copy(rhs), _)
            | Rvalue::Cast(
                CastKind::Pointer(PointerCast::MutToConstPointer),
                Operand::Move(rhs),
                _,
            )
            | Rvalue::Cast(
                CastKind::Pointer(PointerCast::MutToConstPointer),
                Operand::Copy(rhs),
                _,
            ) = rvalue
            {
                match (
                    self.process_ptr_place(place, location),
                    self.process_ptr_place(rhs, location),
                ) {
                    (
                        PtrPlaceDefResult::Base {
                            old: lhs_old,
                            new: lhs_new,
                        },
                        PtrPlaceDefResult::Base {
                            old: rhs_old,
                            new: rhs_new,
                        },
                    ) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(lhs_new.clone().skip(1), rhs_new.clone().skip(1))
                        {
                            self.ctxt.constraint_system.push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt.constraint_system.push_transfer(
                            rhs_old.start,
                            lhs_new.start,
                            rhs_new.start,
                        );
                        // .push_eq(rhs_old.start, [lhs_new.start, rhs_new.start].into_iter());
                        self.ctxt.constraint_system.assume(lhs_old.start, false)
                    }
                    (PtrPlaceDefResult::Base { old, new }, PtrPlaceDefResult::Proj(f)) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(new.clone().skip(1), f.clone().skip(1))
                        {
                            self.ctxt.constraint_system.push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt.constraint_system.push_le(new.start, f.start);
                        // .push_ge(f.start, [new.start].into_iter());
                        self.ctxt.constraint_system.assume(old.start, false);
                    }
                    (PtrPlaceDefResult::Proj(f), PtrPlaceDefResult::Base { old, new }) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(f.clone().skip(1), new.clone().skip(1))
                        {
                            self.ctxt.constraint_system.push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt
                            .constraint_system
                            .push_transfer(old.start, f.start, new.start)
                        // .push_eq(old.start, [f.start, new.start].into_iter());
                    }
                    (PtrPlaceDefResult::Proj(f), PtrPlaceDefResult::Proj(g)) => {
                        for (lhs_inner, rhs_inner) in
                            std::iter::zip(f.clone().skip(1), g.clone().skip(1))
                        {
                            self.ctxt.constraint_system.push_eq(lhs_inner, rhs_inner);
                        }

                        self.ctxt.constraint_system.push_le(f.start, g.start);
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
                        } else if matches!(
                            self.ctxt.tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::Item(_))
                        ) {
                            self.model_boundary(callee_did, args, destination, location);
                            return;
                        }

                        unreachable!()
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

pub struct IntraInferCtxt<'infercx, 'tcx: 'infercx> {
    // func: Func,
    tcx: TyCtxt<'tcx>,
    body: &'infercx Body<'tcx>,
    call_graph: &'infercx CallGraph,
    // rho_ctxt: CrateAnalysisCtxtIntraView<'infercx, Rho, Option<bool>>,
    // inter_ctxt: &'infercx mut InterCtxtView<'inter>,
    field_defs: &'infercx FxHashMap<LocalDefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
    phi_vars_rep: PhiNodeInsertionPoints<Option<Range<Rho>>>,
    return_vars_rep: Option<Range<Rho>>,
    intra_source_map: Vec<LocalSourceInfo>,
    locals: IndexVec<Local, IndexVec<SSAIdx, Range<Rho>>>,
    constraint_system: ConstraintDatabase, //Vec<Constraint>,
}

impl<'infercx, 'tcx> IntraInferCtxt<'infercx, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        // func: Func,
        body: &'infercx Body<'tcx>,
        call_graph: &'infercx CallGraph,
        global_assumptions: ConstraintDatabase,
        field_defs: &'infercx FxHashMap<LocalDefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
        // inter_ctxt: &'infercx mut InterCtxtView<'inter>,
        phi_vars_rep: PhiNodeInsertionPoints<Option<Range<Rho>>>,
    ) -> Self {
        let mut constraint_system = global_assumptions; //inter_ctxt.global_assumptions.clone();
        let mut intra_source_map = Vec::new();
        let locals: IndexVec<_, _> = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                let start = constraint_system.le_constraints.local_start + intra_source_map.len();
                let mut end = start;
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
                        assert_eq!(end, constraint_system.new_var());
                        end = end + 1;
                        intra_source_map.push(LocalSourceInfo {
                            base: local,
                            ssa_idx: SSAIdx::ENTRY,
                            nested_level,
                        });
                    });
                // end = end + 1;
                (end > start)
                    .then(|| IndexVec::from_raw(vec![Range { start, end }]))
                    .unwrap_or_else(|| IndexVec::new())
                // IndexVec::from_raw(vec![Range { start, end }])
            })
            .collect();

        IntraInferCtxt {
            tcx,
            // func,
            body,
            call_graph,
            field_defs,
            // inter_ctxt,
            phi_vars_rep,
            return_vars_rep: None,
            intra_source_map,
            locals,
            constraint_system, //ConstraintDataBase::new(globals, locals),
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
            for (local, rhos) in self.locals.iter_enumerated() {
                //assert_eq!(rhos.len(), 1);
                if !rhos.is_empty() {
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
        }
        self
    }

    fn generate_variables_for_mir_local(&mut self, local: Local, idx: SSAIdx) -> Range<Rho> {
        let entry_rhos = &self.locals[local][SSAIdx::ENTRY];
        let nested_level = entry_rhos.len();

        assert_eq!(self.locals[local].len(), idx.index());

        let start =
            self.intra_source_map.len() + self.constraint_system.le_constraints.local_start.index();

        (0..nested_level).for_each(|nested_level| {
            let rho = self.constraint_system.new_var();
            self.intra_source_map.push(LocalSourceInfo {
                base: local,
                ssa_idx: idx,
                nested_level,
            });
            debug_assert_eq!(
                self.intra_source_map.len(),
                rho.index() - self.constraint_system.le_constraints.local_start.index() + 1
            )
        });

        let end =
            self.intra_source_map.len() + self.constraint_system.le_constraints.local_start.index();

        let res = Range {
            start: start.into(),
            end: end.into(),
        };

        self.locals[local].push(res.clone());

        res
    }
}

impl<'infercx, 'tcx> SSANameHandler for IntraInferCtxt<'infercx, 'tcx> {
    type Output = Range<Rho>;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        log::debug!("IntraInferencer defining {:?}^{} of ptr type", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());

        self.generate_variables_for_mir_local(local, idx)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        log::debug!("IntraInferencer using {:?}^{} of ptr type", local, idx);
        self.locals[local][idx].clone()
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        log::debug!(
            "IntraInferencer at phi node defining {:?}^{} of ptr type",
            local,
            idx
        );
        // self.rho_ctxt.generate_local(local, idx);
        let rhos = self.generate_variables_for_mir_local(local, idx);
        if let Some(rep_rhos) = &self.phi_vars_rep[block][local] {
            for (rep_rho, rho) in std::iter::zip(rep_rhos.clone(), rhos) {
                self.constraint_system.push_eq(rep_rho, rho)
            }
        } else {
            self.phi_vars_rep[block][local] = Some(rhos)
        }
        // self.phi_vars_rep[block][local].push(idx)
    }

    fn handle_use_at_phi_node(
        &mut self,
        local: Local,
        idx: SSAIdx,
        block: BasicBlock,
        _pos: usize,
    ) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        log::debug!(
            "IntraInferencer at phi node using {:?}^{} of ptr type",
            local,
            idx
        );
        let rhos = self.locals[local][idx].clone();
        if let Some(rep_rhos) = &self.phi_vars_rep[block][local] {
            for (rep_rho, rho) in std::iter::zip(rep_rhos.clone(), rhos) {
                self.constraint_system.push_eq(rep_rho, rho)
            }
        } else {
            self.phi_vars_rep[block][local] = Some(rhos)
        }
        // self.phi_vars_rep[block][local].push(idx)
    }
}
