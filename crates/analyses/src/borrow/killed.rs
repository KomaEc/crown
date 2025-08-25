use rustc_borrowck::consumers::{PlaceConflictBias, places_conflict};
use rustc_index::{IndexVec, bit_set::DenseBitSet};
use rustc_middle::{
    mir::{
        Body, Local, Location, Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind,
        visit::Visitor,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::points::{DenseLocationMap, PointIndex};

use crate::borrow::{BorrowSet, Loan};

pub(crate) type Killed = IndexVec<PointIndex, DenseBitSet<Loan>>;

pub fn compute_killed<'tcx>(
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    location_map: &DenseLocationMap,
    borrow_set: &BorrowSet<Place<'tcx>>,
) -> Killed {
    let mut killed = IndexVec::from_elem_n(
        DenseBitSet::new_empty(borrow_set.loans.len()),
        location_map.num_points(),
    );

    let mut collector = KillsCollector {
        killed: &mut killed,
        borrow_set,
        tcx,
        body,
        location_map,
    };
    collector.visit_body(body);

    killed
}

/// Copy-paste of https://doc.rust-lang.org/beta/nightly-rustc/src/rustc_borrowck/polonius/loan_liveness.rs.html#197-207
struct KillsCollector<'kill, 'tcx, P> {
    killed: &'kill mut Killed,
    borrow_set: &'kill BorrowSet<P>,
    tcx: TyCtxt<'tcx>,
    body: &'kill Body<'tcx>,
    location_map: &'kill DenseLocationMap,
}

impl<'kill, 'tcx> KillsCollector<'kill, 'tcx, Place<'tcx>> {
    fn record_killed_borrows_for_place(&mut self, place: Place<'tcx>, location: Location) {
        let other_borrows_of_local = self
            .borrow_set
            .local_map
            .row(place.local)
            .into_iter()
            .flat_map(|bit_set| bit_set.iter());

        let point_index = self.location_map.point_from_location(location);

        // If the borrowed place is a local with no projections, all other borrows of this
        // local must conflict. This is purely an optimization so we don't have to call
        // `places_conflict` for every borrow.
        if place.projection.is_empty() {
            for loan in other_borrows_of_local {
                self.killed[point_index].insert(loan);
            }
            return;
        }

        // By passing `PlaceConflictBias::NoOverlap`, we conservatively assume that any given
        // pair of array indices are not equal, so that when `places_conflict` returns true, we
        // will be assured that two places being compared definitely denotes the same sets of
        // locations.
        let definitely_conflicting_borrows = other_borrows_of_local.filter(|&i| {
            places_conflict(
                self.tcx,
                self.body,
                self.borrow_set.loans[i].path,
                place,
                PlaceConflictBias::NoOverlap,
            )
        });

        for loan in definitely_conflicting_borrows {
            self.killed[point_index].insert(loan);
        }
    }

    fn record_killed_borrows_for_local(&mut self, local: Local, location: Location) {
        let other_borrows_of_local = self
            .borrow_set
            .local_map
            .row(local)
            .into_iter()
            .flat_map(|bit_set| bit_set.iter());

        let point_index = self.location_map.point_from_location(location);

        for loan in other_borrows_of_local {
            self.killed[point_index].insert(loan);
        }
    }
}

impl<'kill, 'tcx> Visitor<'tcx> for KillsCollector<'kill, 'tcx, Place<'tcx>> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        // Make sure there are no remaining borrows for locals that have gone out of scope.
        if let StatementKind::StorageDead(local) = statement.kind {
            self.record_killed_borrows_for_local(local, location);
        }

        self.super_statement(statement, location);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        // When we see `X = ...`, then kill borrows of `(*X).foo` and so forth.
        self.record_killed_borrows_for_place(*place, location);
        self.super_assign(place, rvalue, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        // A `Call` terminator's return value can be a local which has borrows, so we need to record
        // those as killed as well.
        if let TerminatorKind::Call { destination, .. } = terminator.kind {
            self.record_killed_borrows_for_place(destination, location);
        }

        self.super_terminator(terminator, location);
    }
}
