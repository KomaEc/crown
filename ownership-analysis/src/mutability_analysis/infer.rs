pub mod boundary_model;
pub mod libcall_model;

use std::ops::Range;

use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Body, Local, Location, Operand, Place, Rvalue, Statement, Terminator,
        TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;
use smallvec::smallvec;

use super::{
    AnalysisEngine, ConstraintDatabase, IntraSummary, LocalSourceInfo, Mu,
    MutabilityAnalysisBoundary,
};
use crate::{
    boundary_model::BoundaryModel,
    call_graph::CallGraph,
    def_use::{IsDefUse, MutabilityAnalysisDefUse},
    libcall_model::LibCallModel,
    ssa::{
        body_ext::{BodyExt, PhiNodeInsertionPoints},
        rename::{
            handler::{SSADefSites, SSANameSourceMap},
            HasSSANameHandler, HasSSARenameState, HasTyCtxt, SSAIdx, SSANameHandler, SSARename,
            SSARenameState,
        },
    },
    ty_ext::TyExt,
    utils::range_ext::RangeExt,
    FuncSig, Surface,
};

impl<'analysis, 'tcx> AnalysisEngine<'analysis, 'tcx> {
    pub fn infer<Handler: SSANameHandler<Output = ()>>(&mut self, mut extra_handlers: Handler) {
        for (func, &did) in self.call_graph.functions.iter_enumerated() {
            let body = self.tcx.optimized_mir(did);
            let insertion_points = body.compute_phi_node::<MutabilityAnalysisDefUse>(self.tcx);

            let infer_ctxt = IntraInferCtxt::new(
                self.tcx,
                // func,
                body,
                &self.call_graph,
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
                            crate::utils::graph_ext::implementation::forward_star::Direction::Outgoing,
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

            let (func_sig, intra_summary) = infer.complete(|h| (h.1, h.2));

            assert_eq!(self.intra_summaries.push(intra_summary), func);
            assert_eq!(self.func_sigs.push(func_sig), func);
        }
    }
}

pub struct IntraInfer<'infercx, 'tcx: 'infercx, Handler: SSANameHandler> {
    ctxt: IntraInferCtxt<'infercx, 'tcx>,
    ssa_state: SSARenameState<Local>,
    boundaries: Vec<MutabilityAnalysisBoundary>,
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
    type Output = Option<Mu>;

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

    type DefUse = MutabilityAnalysisDefUse;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> HasTyCtxt<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.ctxt.tcx
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> SSARename<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn define_local(&mut self, local: Local, location: Location) -> Option<Mu> {
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

impl<'infercx, 'tcx, Handler: SSANameHandler> IntraInfer<'infercx, 'tcx, Handler> {
    #[inline]
    fn handle_ptr_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Mu {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.handle_def(local, idx, location)
    }

    #[inline]
    fn handle_ptr_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Mu {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.handle_use(local, idx, location)
    }

    #[inline]
    fn use_ptr_local(&mut self, local: Local, location: Location) -> Mu {
        let ssa_idx = self.ssa_state().r#use(local);
        self.handle_ptr_use(local, ssa_idx, location)
    }

    fn define_ptr_local(&mut self, local: Local, location: Location) -> Mu {
        let old = self.use_ptr_local(local, location);
        let new_ssa = self.ssa_state().define(local);
        let new = self.handle_ptr_def(local, new_ssa, location);
        self.ctxt.constraint_system.push_eq(new, old);
        new
    }

    fn complete(
        self,
        f: impl FnOnce(
            Handler,
        ) -> (
            SSANameSourceMap<MutabilityAnalysisDefUse>,
            SSADefSites<MutabilityAnalysisDefUse>,
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

        let ret_mu = return_vars_rep;

        let ret_mus = match ret_mu {
            Some(mu) => Range {
                start: mu,
                end: mu + 1,
            },
            None => RangeExt::empty(),
        };

        let mut surface = vec![smallvec![None; ret_mus.len()]];
        let mut inner = vec![ret_mus];

        for arg in locals.iter().skip(1).take(body.arg_count) {
            if !arg.is_empty() {
                let mu = arg[SSAIdx::ENTRY].clone();
                surface.push(smallvec![None; 1]);
                inner.push(Range {
                    start: mu,
                    end: mu + 1,
                });
            } else {
                surface.push(smallvec![]);
                inner.push(RangeExt::empty());
            }
        }

        tracing::debug!("Generating surface function signature {:?}", &surface);
        tracing::debug!("Generating function signature {:?}", &inner);

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
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = <Self as HasSSANameHandler>::DefUse::categorise_for_local(
            local,
            &self.ctxt.body.local_decls,
            context,
        ) {
            if def_use.defining() {
                self.define_local(local, location);
            } else if def_use.using() {
                let mu = self.use_local(local, location);
                if let Some(mu) = mu {
                    if let PlaceContext::MutatingUse(
                        MutatingUseContext::Projection
                        | MutatingUseContext::Borrow
                        | MutatingUseContext::AddressOf,
                    ) = context
                    {
                        self.ctxt.constraint_system.assume(mu, true);
                    }
                }
            }
        }
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        use colored::*;
        tracing::debug!("{}", &format!("visiting statement {:?}", statement).bold()); //statement);
        self.super_statement(statement, location)
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Some(lhs) = place.as_local() {
            if self.ctxt.body.local_decls[lhs].ty.is_ptr_but_not_fn_ptr() {
                if let Rvalue::Use(Operand::Move(rhs))
                | Rvalue::Use(Operand::Copy(rhs))
                | Rvalue::Cast(_, Operand::Move(rhs), _)
                | Rvalue::Cast(_, Operand::Copy(rhs), _) = rvalue
                {
                    if let Some(rhs) = rhs.as_local() {
                        let lhs = self.define_ptr_local(lhs, location);
                        let rhs = self.use_ptr_local(rhs, location);
                        self.ctxt.constraint_system.push_le(lhs, rhs);
                        return;
                    }
                }
            }
        }
        use colored::*;
        tracing::warn!(
            "{}",
            "this statement is not processed by the intra inferencer"
                .bold()
                .red()
        );
        self.super_assign(place, rvalue, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        use colored::*;
        tracing::debug!(
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
            target,
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
                            self.model_libc_call(
                                callee_did,
                                args,
                                target.map(|tgt| (destination, tgt)),
                                location,
                            );
                            return;
                        } else if matches!(
                            self.ctxt.tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::Item(_))
                        ) {
                            self.model_boundary(
                                callee_did,
                                args,
                                target.map(|tgt| (destination, tgt)),
                                location,
                            );
                            return;
                        }
                    }
                    // library functions
                    None => {
                        self.model_library_call(
                            callee_did,
                            args,
                            target.map(|tgt| (destination, tgt)),
                            location,
                        );
                        return;
                    }
                }
            } else {
                unreachable!("what could it be? {}", ty)
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
    phi_vars_rep: PhiNodeInsertionPoints<Option<Mu>>,
    return_vars_rep: Option<Mu>,
    intra_source_map: Vec<LocalSourceInfo>,
    locals: IndexVec<Local, IndexVec<SSAIdx, Mu>>,
    constraint_system: ConstraintDatabase, //Vec<Constraint>,
}

impl<'infercx, 'tcx> IntraInferCtxt<'infercx, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        body: &'infercx Body<'tcx>,
        call_graph: &'infercx CallGraph,
        phi_vars_rep: PhiNodeInsertionPoints<Option<Mu>>,
    ) -> Self {
        let mut constraint_system = ConstraintDatabase::new(0, 0);
        let mut intra_source_map = Vec::new();
        let locals: IndexVec<_, _> = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                if local_decl.ty.is_ptr_but_not_fn_ptr() {
                    intra_source_map.push(LocalSourceInfo {
                        base: local,
                        ssa_idx: SSAIdx::ENTRY,
                        nested_level: 0,
                    });
                    IndexVec::from_raw(vec![constraint_system.new_var()])
                } else {
                    IndexVec::new()
                }
            })
            .collect();
        IntraInferCtxt {
            tcx,
            // func,
            body,
            call_graph,
            // inter_ctxt,
            phi_vars_rep,
            return_vars_rep: None,
            intra_source_map,
            locals,
            constraint_system, //ConstraintDataBase::new(globals, locals),
        }
    }

    fn generate_variables_for_mir_local(&mut self, local: Local, idx: SSAIdx) -> Mu {
        let mu = self.constraint_system.new_var();
        assert_eq!(self.locals[local].push(mu), idx);
        mu
    }
}

impl<'infercx, 'tcx> SSANameHandler for IntraInferCtxt<'infercx, 'tcx> {
    type Output = Mu;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        tracing::debug!("IntraInferencer defining {:?}^{} of ptr type", local, idx);
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());

        self.generate_variables_for_mir_local(local, idx)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, _location: Location) -> Self::Output {
        tracing::debug!("IntraInferencer using {:?}^{} of ptr type", local, idx);
        self.locals[local][idx].clone()
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        debug_assert!(self.body.local_decls[local].ty.is_ptr_but_not_fn_ptr());
        tracing::debug!(
            "IntraInferencer at phi node defining {:?}^{} of ptr type",
            local,
            idx
        );
        // self.rho_ctxt.generate_local(local, idx);
        let mu = self.generate_variables_for_mir_local(local, idx);
        if let &Some(rep_mu) = &self.phi_vars_rep[block][local] {
            self.constraint_system.push_eq(rep_mu, mu)
        } else {
            self.phi_vars_rep[block][local] = Some(mu)
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
        tracing::debug!(
            "IntraInferencer at phi node using {:?}^{} of ptr type",
            local,
            idx
        );
        let mu = self.locals[local][idx].clone();
        if let &Some(rep_mu) = &self.phi_vars_rep[block][local] {
            self.constraint_system.push_eq(rep_mu, mu)
        } else {
            self.phi_vars_rep[block][local] = Some(mu)
        }
        // self.phi_vars_rep[block][local].push(idx)
    }
}
