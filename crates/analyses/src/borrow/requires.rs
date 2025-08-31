use rustc_index::{
    IndexVec,
    bit_set::{DenseBitSet, SparseBitMatrix},
};
use rustc_middle::mir::Body;
use utils::smallvec::{SmallVec, smallvec};

use crate::borrow::{
    BorrowSet, Loan, MembershipConstraint, Provenance, ProvenanceConstraintGraph, ProvenanceSet,
    SubsetConstraint,
};

pub(crate) type ProvenanceRequiresLoan = SparseBitMatrix<Provenance, Loan>;

pub fn compute_requires<'tcx>(
    body: &Body<'tcx>,
    borrow_set: &BorrowSet<'tcx>,
    provenance_set: &ProvenanceSet,
) -> ProvenanceRequiresLoan {
    let constraint_graph = ProvenanceConstraintGraph::new(body, borrow_set, provenance_set);
    constraint_graph.solve(borrow_set, provenance_set)
}

impl ProvenanceConstraintGraph {
    fn solve(
        &self,
        borrow_set: &BorrowSet,
        provenance_set: &ProvenanceSet,
    ) -> ProvenanceRequiresLoan {
        let mut subset_graph = IndexVec::<Provenance, SmallVec<[Provenance; 4]>>::from_elem(
            smallvec![],
            &provenance_set.provenance_data,
        );

        for SubsetConstraint { sup, sub, .. } in self.subset.iter().copied() {
            subset_graph[sub].push(sup);
        }

        let mut answer = SparseBitMatrix::new(borrow_set.loans.len());

        let mut stack: Vec<Provenance> = vec![];
        let mut visited: DenseBitSet<Provenance> =
            DenseBitSet::new_empty(provenance_set.provenance_data.len());

        for MembershipConstraint { loan, provenance } in self.membership.iter().copied() {
            stack.clear();
            visited.clear();

            stack.push(provenance);

            while let Some(provenance) = stack.pop() {
                if !visited.insert(provenance) {
                    continue;
                }
                answer.insert(provenance, loan);
                stack.extend_from_slice(&subset_graph[provenance]);
            }
        }

        answer
    }
}
