//! Dynamically verify several assumptions we made on rustc.
#![feature(rustc_private)]
#![feature(box_patterns)]

extern crate rustc_index;
extern crate rustc_middle;

use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        Body, Local, LocalInfo, LocalKind, Location, Operand, Place, Rvalue, Terminator,
        TerminatorKind,
    },
    ty::TyCtxt,
};

pub fn verify(krate: &common::CrateData) {
    verify_shape_of_place(krate);
    verify_place_regularity(krate);
    verify_temp_local_usage(krate);
    verify_args_are_all_locals(krate);
    verify_stmt_regularity(krate);
    verify_return_clause_unique(krate);
    verify_projection_elem_intern(krate);
}

fn verify_shape_of_place(krate: &common::CrateData) {
    struct Vis;
    impl<'tcx> Visitor<'tcx> for Vis {
        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _location: Location) {
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
    for did in &krate.fns {
        let body = krate.tcx.optimized_mir(did);
        Vis.visit_body(body);
    }
}

fn verify_place_regularity(krate: &common::CrateData) {
    struct Vis<'me, 'tcx>(&'me Body<'tcx>, TyCtxt<'tcx>);
    impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
        fn visit_assign(
            &mut self,
            place: &Place<'tcx>,
            rvalue: &Rvalue<'tcx>,
            _location: Location,
        ) {
            // we only have the following cases:
            // local = use(operand)
            // place = use(local)
            // local = cast(local)
            // local = &place
            // local = deref_copy place
            match rvalue {
                Rvalue::Use(operand) => {
                    if !place.ty(self.0, self.1).ty.is_primitive() {
                        assert!(
                            place.as_local().is_some()
                                || operand.constant().is_some()
                                || operand.place().and_then(|place| place.as_local()).is_some()
                        )
                    }
                }
                Rvalue::Cast(_, operand, _) => {
                    assert!(
                        operand.place().and_then(|place| place.as_local()).is_some()
                            || operand.constant().is_some()
                    )
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

        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _location: Location) {
            let PlaceContext::MutatingUse(MutatingUseContext::Call) = context else { return };
            assert!(place.as_local().is_some())
        }
    }

    for &did in &krate.fns {
        let body = krate.tcx.optimized_mir(did);
        Vis(body, krate.tcx).visit_body(body);
    }
}

fn verify_args_are_all_locals(krate: &common::CrateData) {
    struct Vis;
    impl<'tcx> Visitor<'tcx> for Vis {
        fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _: Location) {
            let TerminatorKind::Call { destination, args, .. } = &terminator.kind else { return };
            for arg in args {
                match arg {
                    Operand::Move(place) => assert!(place.as_local().is_some()),
                    Operand::Copy(..) => unreachable!(),
                    _ => {}
                }
            }
            assert!(destination.as_local().is_some());
        }
    }
    for did in &krate.fns {
        let body = krate.tcx.optimized_mir(did);
        Vis.visit_body(body);
    }
}

fn verify_temp_local_usage(krate: &common::CrateData) {
    use rustc_index::vec::IndexVec;
    for did in &krate.fns {
        let body = krate.tcx.optimized_mir(did);
        let mut cnt = IndexVec::from_elem(0, &body.local_decls);
        struct Vis<'me>(&'me mut IndexVec<Local, usize>);
        impl<'me, 'tcx> Visitor<'tcx> for Vis<'me> {
            fn visit_local(&mut self, local: Local, context: PlaceContext, _location: Location) {
                if matches!(
                    context,
                    PlaceContext::MutatingUse(MutatingUseContext::Call | MutatingUseContext::Store)
                ) {
                    self.0[local] += 1;
                }
            }
        }
        Vis(&mut cnt).visit_body(body);

        for local in body.local_decls.indices() {
            match body.local_kind(local) {
                LocalKind::Var => {
                    // assert!(cnt[local] >= 1, "{}:{:?}", krate.tcx.def_path_str(body.source.def_id()), local)
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

fn verify_stmt_regularity(krate: &common::CrateData) {
    for did in krate.fns.iter().copied() {
        let body = krate.tcx.optimized_mir(did);
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
                        | rustc_middle::mir::StatementKind::Intrinsic(..)
                ) {
                    panic!("unexpected stmt kind {:?} at {:?}", statement.kind, did)
                }
            }
        }
    }
}

fn verify_return_clause_unique(krate: &common::CrateData) {
    for did in krate.fns.iter().copied() {
        let body = krate.tcx.optimized_mir(did);
        assert!(
            body.basic_blocks
                .iter()
                .filter(|bb_data| matches!(bb_data
                    .terminator
                    .as_ref(),
                    Some(terminator) if matches!(terminator.kind, TerminatorKind::Return)))
                .count()
                <= 1
        );
    }
}

fn verify_projection_elem_intern(krate: &common::CrateData) {
    struct Vis<'tcx>(TyCtxt<'tcx>);
    impl<'tcx> Visitor<'tcx> for Vis<'tcx> {
        fn visit_place(&mut self, place: &Place<'tcx>, _: PlaceContext, _: Location) {
            let tcx = self.0;
            let projection = &place.projection[..];
            let new_place = Place::project_deeper(Place::from(place.local), projection, tcx);
            assert_eq!(*place, new_place);
        }
    }
    for did in krate.fns.iter().copied() {
        let body = krate.tcx.optimized_mir(did);
        Vis(krate.tcx).visit_body(body);
    }
}
