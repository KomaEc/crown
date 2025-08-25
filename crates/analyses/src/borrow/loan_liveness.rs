use rustc_index::bit_set::{DenseBitSet, SparseBitMatrix};
use rustc_middle::{
    mir::{Body, Location, Place, Statement, Terminator, TerminatorEdges},
    ty::TyCtxt,
};
use rustc_mir_dataflow::{
    Analysis,
    points::{DenseLocationMap, PointIndex},
};

use crate::borrow::{
    BorrowSet, Loan, ProvenanceSet, killed::Killed, provenance_liveness::ProvenanceLiveness,
    requires::ProvenanceRequiresLoan,
};

/// The set of program points that a [`Loan`] is live on entry
pub(crate) type LoanLiveness = SparseBitMatrix<PointIndex, Loan>;

pub fn compute_loan_liveness<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    borrow_set: &BorrowSet<Place<'tcx>>,
    provenance_set: &ProvenanceSet,
    location_map: &DenseLocationMap,
    provenance_liveness: &ProvenanceLiveness,
    requires: &ProvenanceRequiresLoan,
    killed: &Killed,
) -> LoanLiveness {
    let mut loan_liveness = SparseBitMatrix::new(borrow_set.loans.len());

    let mut loan_live_at = LoanLiveAt {
        borrow_set,
        provenance_set,
        location_map,
        provenance_liveness,
        requires,
        killed,
    }
    .iterate_to_fixpoint(tcx, body, None)
    .into_results_cursor(body);

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        loan_live_at.seek_to_block_start(bb);

        let bb_len = bb_data.statements.len() + bb_data.terminator.is_some() as usize;
        for position in 0..bb_len {
            let location = Location {
                block: bb,
                statement_index: position,
            };

            loan_live_at.seek_before_primary_effect(location);
            let liveness = loan_live_at.get();
            if !liveness.is_empty() {
                let point_index = location_map.point_from_location(location);
                loan_liveness.union_row(point_index, liveness);
            }
        }
    }

    loan_liveness
}

pub struct LoanLiveAt<'analysis, 'tcx> {
    borrow_set: &'analysis BorrowSet<Place<'tcx>>,
    provenance_set: &'analysis ProvenanceSet,

    location_map: &'analysis DenseLocationMap,
    provenance_liveness: &'analysis ProvenanceLiveness,
    requires: &'analysis ProvenanceRequiresLoan,
    killed: &'analysis Killed,
}

impl<'analysis, 'tcx> LoanLiveAt<'analysis, 'tcx> {
    fn apply_location_effect(&mut self, state: &mut DenseBitSet<Loan>, location: Location) {
        let point_index = self.location_map.point_from_location(location);

        let killed = &self.killed[point_index];

        let live_provenances = self
            .provenance_liveness
            .row(point_index)
            .into_iter()
            .flat_map(|bit_set| bit_set.iter());

        let mut requires = DenseBitSet::new_empty(killed.domain_size());

        for provenance in self
            .provenance_liveness
            .row(point_index)
            .into_iter()
            .flat_map(|bit_set| bit_set.iter())
        {
            if let Some(loans) = self.requires.row(provenance) {
                requires.union(loans);
            }
        }

        state.intersect(&requires);
        state.subtract(killed);

        if let Some(&loan) = self.borrow_set.location_map.get(&location) {
            state.insert(loan);
        }
    }
}

impl<'analysis, 'tcx> Analysis<'tcx> for LoanLiveAt<'analysis, 'tcx> {
    type Domain = DenseBitSet<Loan>;

    type Direction = rustc_mir_dataflow::Forward;

    const NAME: &'static str = "loan_live_at";

    fn bottom_value(&self, body: &Body<'tcx>) -> Self::Domain {
        DenseBitSet::new_empty(self.borrow_set.loans.len())
    }

    fn initialize_start_block(&self, _: &Body<'tcx>, _: &mut Self::Domain) {}

    fn apply_primary_statement_effect(
        &mut self,
        state: &mut Self::Domain,
        _statement: &Statement<'tcx>,
        location: Location,
    ) {
        self.apply_location_effect(state, location);
    }

    fn apply_primary_terminator_effect<'mir>(
        &mut self,
        state: &mut Self::Domain,
        terminator: &'mir Terminator<'tcx>,
        location: Location,
    ) -> TerminatorEdges<'mir, 'tcx> {
        self.apply_location_effect(state, location);
        terminator.edges()
    }
}
