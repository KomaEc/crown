use rustc_index::bit_set::SparseBitMatrix;
use rustc_middle::{
    mir::{Body, Location},
    ty::TyCtxt,
};
use rustc_mir_dataflow::{
    Analysis,
    points::{DenseLocationMap, PointIndex},
};

use crate::{
    borrow::{Provenance, ProvenanceSet},
    liveness::MaybeLiveLocals,
};

/// The set of program points that a [`Provenance`] is live on exit
pub(crate) type ProvenanceLiveness = SparseBitMatrix<PointIndex, Provenance>;

pub fn compute_provenance_liveness<'tcx>(
    location_map: &DenseLocationMap,
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    provenance_set: &ProvenanceSet,
) -> ProvenanceLiveness {
    let mut provenance_liveness = ProvenanceLiveness::new(provenance_set.provenance_data.len());

    let mut local_liveness = MaybeLiveLocals
        .iterate_to_fixpoint(tcx, body, None)
        .into_results_cursor(body);
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        local_liveness.seek_to_block_end(bb);

        let bb_len = bb_data.statements.len() + bb_data.terminator.is_some() as usize;
        for position in (0..bb_len).rev() {
            let location = Location {
                block: bb,
                statement_index: position,
            };

            let point_index = location_map.point_from_location(location);

            local_liveness.seek_before_primary_effect(location);
            let liveness = local_liveness.get();
            for provenance in liveness
                .iter()
                .flat_map(|local| provenance_set.local_data[local])
            {
                provenance_liveness.insert(point_index, provenance);
            }
        }
    }

    provenance_liveness
}
