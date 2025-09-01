use rustc_index::{
    IndexVec,
    bit_set::{DenseBitSet, SparseBitMatrix},
};
use utils::smallvec::{SmallVec, smallvec};

use crate::borrow::{Provenance, ProvenanceConstraintGraph, ProvenanceSet, SubsetConstraint};

pub fn compute_subset_closure(
    provenance_set: &ProvenanceSet,
    constraint_graph: &ProvenanceConstraintGraph,
) -> SubSetClosure {
    constraint_graph.compute_subset_closure(provenance_set)
}

pub(crate) type SubSetClosure = SparseBitMatrix<Provenance, Provenance>;

impl ProvenanceConstraintGraph {
    fn compute_subset_closure(&self, provenance_set: &ProvenanceSet) -> SubSetClosure {
        let mut subset_graph = IndexVec::<Provenance, SmallVec<[Provenance; 4]>>::from_elem(
            smallvec![],
            &provenance_set.provenance_data,
        );

        for SubsetConstraint { sup, sub, .. } in self.subset.iter().copied() {
            subset_graph[sub].push(sup);
        }

        let mut answer = SparseBitMatrix::new(provenance_set.provenance_data.len());

        let mut stack: Vec<Provenance> = vec![];
        let mut visited: DenseBitSet<Provenance> =
            DenseBitSet::new_empty(provenance_set.provenance_data.len());

        for provenance in provenance_set.provenance_data.indices() {
            stack.clear();
            visited.clear();

            stack.push(provenance);

            while let Some(other_provenance) = stack.pop() {
                if !visited.insert(other_provenance) {
                    continue;
                }
                answer.insert(provenance, other_provenance);
                stack.extend_from_slice(&subset_graph[provenance]);
            }
        }

        answer
    }
}
