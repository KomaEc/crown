use rustc_index::bit_set::SparseBitMatrix;
use rustc_mir_dataflow::points::PointIndex;

use crate::borrow::{BorrowSet, Loan, invalidates::Invalidates, loan_liveness::LoanLiveness};

pub(crate) type Errors = SparseBitMatrix<PointIndex, Loan>;

pub fn compute_errors(
    borrow_set: &BorrowSet,
    loan_liveness: &LoanLiveness,
    invalidates: &Invalidates,
) -> Errors {
    let mut answer = SparseBitMatrix::new(borrow_set.loans.len());

    for row in loan_liveness.rows() {
        if let (Some(invalidates_at), Some(loan_live_at)) =
            (invalidates.row(row), loan_liveness.row(row))
        {
            answer.union_row(row, loan_live_at);
            answer.intersect_row(row, invalidates_at);
        }
    }

    answer
}
