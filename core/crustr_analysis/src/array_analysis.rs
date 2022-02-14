//! Global effect: function summary and field def summary -> interprocedural analysis
//! Local effect: local value -> intraprocedural data flow analysis
//! The question to think about: in the buffer example, why buffer_t.alloc
//! is a slice and why buffer_t.data is not? If we treat it as an equational
//! value flow (bidirectional), now that buffer_t.data is ever assigned by
//! buffer_t.alloc, how can we block this flow?
//! Or if we must treat it as directional value flow? Flow from fat to thin
//! only!!!!

use rustc_middle::mir::Place;

pub mod intra;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary;

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}

pub struct LambdaData<'tcx> {
    pub place: Place<'tcx>,
    pub idx: usize,
}

impl<'tcx> From<(Place<'tcx>, usize)> for LambdaData<'tcx> {
    fn from((place, idx): (Place<'tcx>, usize)) -> Self {
        LambdaData { place, idx }
    }
}

pub enum Constraint {
    /// l = 1
    AssertFat(Lambda),
    /// l = 0
    AssertThin(Lambda),
    /// l1 <= l2
    ThinerThan(Lambda, Lambda),
}
