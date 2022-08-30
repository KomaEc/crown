//! Dynamically verify several assumptions we made on rustc.

use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    Local, LocalInfo, LocalKind, Location, Place, Rvalue, Terminator, TerminatorKind,
};

use crate::CrateInfo;

impl<'tcx> CrateInfo<'tcx> {
    pub fn verify(&self) {
        self.verify_shape_of_place();
        self.verify_place_regularity();
        self.verify_temp_local_usage();
        self.verify_args_are_all_locals();
        self.verify_stmt_regularity()
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

    // pub fn compute_percentage_of_non_address_taking_functions(&self) -> Result<()> {
    //     struct Vis;
    //     impl<'tcx> Visitor<'tcx> for Vis {
    //         fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, _location: Location) {
    //             if let Rvalue::AddressOf(rustc_ast::Mutability::Mut, _) = rvalue {
    //                 panic!("{:?} to be catched", rvalue)
    //             }
    //         }
    //     }
    //     let prev_hook = std::panic::take_hook();
    //     std::panic::set_hook(Box::new(|_| {}));
    //     let n_address_taking_functions = self
    //         .functions()
    //         .iter()
    //         .filter(|&did| {
    //             let body = self.tcx.optimized_mir(did);
    //             std::panic::catch_unwind(std::panic::assertUnwindSafe(|| Vis.visit_body(body)))
    //                 .is_ok()
    //         })
    //         .count();
    //     std::panic::set_hook(prev_hook);
    //     let percentage =
    //         n_address_taking_functions as f64 / self.call_graph.function_count() as f64;
    //     println!("-------------stat: percntage of non address taking functions-----------------");
    //     println!("                   {percentage}");
    //     Ok(())
    // }

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
                        assert!(!operand.constant().is_some() || place.as_local().is_some());
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

    // pub fn inspect_place_abs(&self) {
    //     struct Vis<'me, 'tcx>(
    //         &'me OwnershipAnalysisCtxt<'me, 'tcx>,
    //         &'me Body<'tcx>,
    //         FxHashSet<Place<'tcx>>,
    //     );
    //     impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
    //         fn visit_place(
    //             &mut self,
    //             place: &Place<'tcx>,
    //             context: PlaceContext,
    //             _location: Location,
    //         ) {
    //             let visited = &mut self.2;
    //             if visited.contains(&place) {
    //                 return;
    //             }
    //             visited.insert(*place);
    //             let octxt = self.0;
    //             let body = self.1;
    //             let (PlaceContext::MutatingUse(..) | PlaceContext::NonMutatingUse(..)) = context else { return };
    //             if place.projection.len() < 2 {
    //                 return;
    //             }
    //             let Some(place_abs) = place.r#abstract(body, &octxt) else { return };
    //             tracing::debug!("{:?} -> {}", place, place_abs)
    //         }
    //     }
    //     let octxt = OwnershipAnalysisCtxt::new(&*self);
    //     for did in self.functions() {
    //         let body = self.tcx.optimized_mir(did);
    //         let mut vis = Vis(&octxt, body, FxHashSet::default());
    //         vis.visit_body(body);
    //     }
    // }

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
}
