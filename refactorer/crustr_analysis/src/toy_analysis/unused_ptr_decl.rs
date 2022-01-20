mod diagnostics;

use crate::pointer_analysis::{andersen::AndersenResult, PointerAnalysisNodeData};
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        Body, Location, Place, PlaceRef, SourceInfo, Statement, VarDebugInfo,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::{
    impls::{MaybeBorrowedLocals, MaybeLiveLocals},
    Analysis, ResultsCursor,
};

pub fn liveness_result<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
) -> ResultsCursor<'a, 'tcx, MaybeLiveLocals> {
    let liveness = MaybeLiveLocals
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
        .into_results_cursor(body);
    liveness
}

pub fn maybe_borrowed_result<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
) -> ResultsCursor<'a, 'tcx, MaybeBorrowedLocals> {
    let maybe_borrowed = MaybeBorrowedLocals::all_borrows()
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
        .into_results_cursor(body);
    maybe_borrowed
}

pub struct UnusedPointerDecl<'upd, 'tcx> {
    bodies: &'upd [&'upd Body<'tcx>],
    tcx: TyCtxt<'tcx>,
    andersen_result: AndersenResult<'upd, 'tcx>,
}

impl<'upd, 'tcx> UnusedPointerDecl<'upd, 'tcx> {
    pub fn new(
        bodies: &'upd [&'upd Body<'tcx>],
        tcx: TyCtxt<'tcx>,
        andersen_result: AndersenResult<'upd, 'tcx>,
    ) -> Self {
        UnusedPointerDecl {
            bodies,
            tcx,
            andersen_result,
        }
    }

    pub fn analyze(self) {
        for &body in self.bodies {
            let mut liveness = liveness_result(self.tcx, body);
            let mut maybe_borrowed = maybe_borrowed_result(self.tcx, body);
            UnusedPointerDeclForBody {
                body,
                tcx: self.tcx,
                andersen_result: &self.andersen_result,
                liveness: &mut liveness,
                maybe_borrowed: &mut maybe_borrowed,
                user_vars: Vec::new(),
                current_stmt: None,
            }
            .initiate_tracked()
            .visit_body(body);
        }
    }
}

pub struct UnusedPointerDeclForBody<'updb, 'upd, 'tcx> {
    body: &'updb Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    andersen_result: &'updb AndersenResult<'upd, 'tcx>,
    liveness: &'updb mut ResultsCursor<'updb, 'tcx, MaybeLiveLocals>,
    maybe_borrowed: &'updb mut ResultsCursor<'updb, 'tcx, MaybeBorrowedLocals>,
    user_vars: Vec<(PlaceRef<'tcx>, &'updb VarDebugInfo<'tcx>)>,
    current_stmt: Option<SourceInfo>,
}

impl<'updb, 'upd, 'tcx> Visitor<'tcx> for UnusedPointerDeclForBody<'updb, 'upd, 'tcx> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        let Statement { source_info, .. } = statement;

        self.current_stmt = Some(source_info.clone());
        self.super_statement(statement, location);
        self.current_stmt = None;
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        let place_ty = place.ty(self.body, self.tcx).ty;
        if matches!(
            self.user_vars
                .binary_search_by_key(&place.as_ref(), |(place_ref, _)| *place_ref),
            Ok(_)
        ) && place_ty.is_any_ptr()
            && !place_ty.is_fn_ptr()
        {
            match context {
                PlaceContext::MutatingUse(_mut_context) => {
                    let p = self
                        .andersen_result
                        .ptr_ctxt
                        .lookup_local(
                            self.body.source.instance.def_id().as_local().unwrap(),
                            place.local,
                        )
                        .expect("place should be generated");

                    assert!(matches!(
                        self.andersen_result.ptr_ctxt.find(p),
                        PointerAnalysisNodeData::Mir(_, _)
                    ));

                    // assert_eq!(p_place_ref.local, place.local);

                    let p_place_ref = Place::from(place.local).as_ref();

                    if let Ok(p_idx) = self
                        .user_vars
                        .binary_search_by_key(&p_place_ref, |(place_ref, _)| *place_ref)
                    {
                        self.liveness.seek_after_primary_effect(location);
                        self.maybe_borrowed.seek_after_primary_effect(location);
                        let liveness = self.liveness.get();
                        // let maybe_borrowed = self.maybe_borrowed.get();

                        for q in self.andersen_result.pts_graph.aliases_for(p) {
                            let q_data = self.andersen_result.ptr_ctxt.find(q);
                            if let PointerAnalysisNodeData::Mir(_, q_place_ref) = q_data {
                                if let Ok(q_idx) = self
                                    .user_vars
                                    .binary_search_by_key(&q_place_ref, |(place_ref, _)| *place_ref)
                                {
                                    if liveness.contains(q_place_ref.local)
                                    // || maybe_borrowed.contains(q_place_ref.local)
                                    {
                                        let (_, p_var_debug) = self.user_vars[p_idx];
                                        let (_, q_var_debug) = self.user_vars[q_idx];

                                        let mut err = self.tcx.sess.struct_span_err(
                                            p_var_debug.source_info.span,
                                            &format!(
                                                "Use of pointer [`{}`] invalidates another pointer [`{}`]",
                                                p_var_debug.name, q_var_debug.name
                                            ),
                                        );

                                        if let Some(ref source_info) = self.current_stmt {
                                            err.span_label(source_info.span, "The assignment statement that causes the invalidation");
                                        }

                                        err.span_label(
                                            p_var_debug.source_info.span,
                                            "First pointer defined here",
                                        );
                                        err.span_label(
                                            q_var_debug.source_info.span,
                                            "Second pointer defined here",
                                        );

                                        err.emit()
                                    }
                                }
                            }
                        }
                    }
                }
                PlaceContext::NonMutatingUse(_) | PlaceContext::NonUse(_) => {}
            }
        }
        self.super_place(place, context, location)
    }
}
