//! Dynamically verify several assumptions we made on rustc.

use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    Local, LocalInfo, LocalKind, Location, Place, Rvalue, Terminator, TerminatorKind,
};

use crate::CrateCtxt;

impl<'tcx> CrateCtxt<'tcx> {
    pub fn verify(&self) {
        self.verify_shape_of_place();
        self.verify_place_regularity();
        self.verify_temp_local_usage();
        self.verify_args_are_all_locals();
        self.verify_stmt_regularity();
        self.verify_return_clause_unique();
        #[cfg(debug_assertions)]
        self.compute_max_num_scc();
    }

    fn verify_shape_of_place(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                _location: Location,
            ) {
                let (PlaceContext::MutatingUse(..) | PlaceContext::NonMutatingUse(..)) = context else { return };

                let mut derefed = false;
                let mut offsetted = false;

                for projection_elem in place.projection {
                    match projection_elem {
                        rustc_middle::mir::ProjectionElem::Deref => {
                            if derefed {
                                panic!("nested derefs found: {:?}", place)
                            }
                            if offsetted {
                                panic!("deref offseted place: {:?}", place)
                            }
                            derefed = true;
                        }
                        rustc_middle::mir::ProjectionElem::Field(_, _) => {
                            offsetted = true;
                        }
                        _ => continue,
                    }
                }
            }
        }
        for did in self.functions() {
            let body = self.tcx.optimized_mir(did);
            Vis.visit_body(body);
        }
    }

    fn verify_place_regularity(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_assign(
                &mut self,
                place: &Place<'tcx>,
                rvalue: &Rvalue<'tcx>,
                _location: Location,
            ) {
                // we only have the following cases:
                // local = place
                // local = constant
                // place = local
                // local = &place
                // local = deref_copy place
                match rvalue {
                    Rvalue::Use(operand) | Rvalue::Cast(_, operand, _) => {
                        assert!(
                            place.as_local().is_some()
                                || operand.place().and_then(|place| place.as_local()).is_some()
                                || operand.constant().is_some()
                        );
                        assert!(operand.constant().is_none() || place.as_local().is_some());
                    }
                    Rvalue::CopyForDeref(rplace) => {
                        assert!(place.as_local().is_some());
                        assert!(rplace.as_local().is_none());
                    }
                    Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                        assert!(place.as_local().is_some())
                    }
                    _ => {}
                }
            }

            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                _location: Location,
            ) {
                let PlaceContext::MutatingUse(MutatingUseContext::Call) = context else { return };
                assert!(place.as_local().is_some())
            }
        }
    }

    fn verify_args_are_all_locals(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _: Location) {
                let TerminatorKind::Call { destination, args, .. } = &terminator.kind else { return };
                for arg in args {
                    assert!(
                        arg.place().and_then(|place| place.as_local()).is_some()
                            || arg.constant().is_some()
                    );
                }
                assert!(destination.as_local().is_some());
            }
        }
        for did in self.functions() {
            let body = self.tcx.optimized_mir(did);
            Vis.visit_body(body);
        }
    }

    fn verify_temp_local_usage(&self) {
        use rustc_index::vec::IndexVec;
        for did in self.functions() {
            let body = self.tcx.optimized_mir(did);
            let mut cnt = IndexVec::from_elem(0, &body.local_decls);
            // for cnt in &mut cnt.raw[1..1+body.arg_count] {
            //     *cnt += 1;
            // }
            struct Vis<'me>(&'me mut IndexVec<Local, usize>);
            impl<'me, 'tcx> Visitor<'tcx> for Vis<'me> {
                fn visit_local(
                    &mut self,
                    local: Local,
                    context: PlaceContext,
                    _location: Location,
                ) {
                    if matches!(
                        context,
                        PlaceContext::MutatingUse(
                            MutatingUseContext::Call | MutatingUseContext::Store
                        )
                    ) {
                        self.0[local] += 1;
                    }
                }
            }
            Vis(&mut cnt).visit_body(body);

            for local in body.local_decls.indices() {
                match body.local_kind(local) {
                    LocalKind::Var => {
                        // assert!(cnt[local] >= 1, "{}:{:?}", self.tcx.def_path_str(body.source.def_id()), local)
                    }
                    LocalKind::Temp => {
                        assert!(
                            !matches!(
                                body.local_decls[local].local_info,
                                Some(box LocalInfo::DerefTemp)
                            ) || cnt[local] == 1,
                            "{:?}:{:?}",
                            body.source.def_id(),
                            local
                        )
                    }
                    LocalKind::Arg => {}
                    LocalKind::ReturnPointer => {}
                }
            }
        }
    }

    fn verify_stmt_regularity(&self) {
        for did in self.functions().iter().copied() {
            let body = self.tcx.optimized_mir(did);
            for bb_data in body.basic_blocks.iter() {
                let rustc_middle::mir::BasicBlockData {
                    statements,
                    terminator: _,
                    ..
                } = bb_data;
                for statement in statements {
                    if !matches!(
                        statement.kind,
                        rustc_middle::mir::StatementKind::Assign(..)
                        // this happens when initialising aggregates
                        | rustc_middle::mir::StatementKind::Deinit(..)
                        // this happens when dealing with fn_ptr, which is wrapped
                        // in an Option type
                        | rustc_middle::mir::StatementKind::SetDiscriminant { .. }
                    ) {
                        panic!("unexpected stmt kind {:?} at {:?}", statement.kind, did)
                    }
                }
                // for terminator in terminator {
                //     if !matches!(
                //         terminator.kind,
                //         rustc_middle::mir::TerminatorKind::Call {..}
                //         | rustc_middle::mir::TerminatorKind::Return
                //         | rustc_middle::mir::TerminatorKind::SwitchInt { .. }
                //     ) {
                //         panic!("unexpected terminator kind {:?} at {:?}", terminator.kind, did)
                //     }
                // }
            }
        }
    }

    fn verify_return_clause_unique(&self) {
        for did in self.functions().iter().copied() {
            let body = self.tcx.optimized_mir(did);
            assert!(
                body.basic_blocks
                    .iter()
                    .filter(
                        |bb_data| bb_data.terminator.is_some_and(|terminator| matches!(
                            terminator.kind,
                            TerminatorKind::Return
                        ))
                    )
                    .count()
                    <= 1
            );
        }
    }

    #[cfg(debug_assertions)]
    fn compute_max_num_scc(&self) {
        let max_num = self
            .call_graph
            .sccs()
            .map(|scc| scc.len())
            .reduce(std::cmp::max)
            .unwrap();
        println!("max scc: {max_num}")
    }
}
