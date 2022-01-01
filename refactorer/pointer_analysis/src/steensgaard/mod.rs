//! Steensgaard's pointer analysis algorithm.

pub mod constraint_solving;

use union_find::{QuickUnionUf, Union, UnionByRank, UnionResult};

use crate::ctxt::PointerAnalysisCtxt;

pub struct SteensgaardResult<'sr, 'tcx> {
    pub locations: QuickUnionUf<AbstractLocation>,
    pub ptr_ctxt: PointerAnalysisCtxt<'sr, 'tcx>,
}

#[derive(Clone, Copy, Default)]
pub struct AbstractLocation {
    rank: UnionByRank,
}

impl Union for AbstractLocation {
    fn union(lval: Self, rval: Self) -> UnionResult<Self> {
        match Union::union(lval.rank, rval.rank) {
            UnionResult::Left(l) => UnionResult::Left(AbstractLocation { rank: l }),
            UnionResult::Right(r) => UnionResult::Left(AbstractLocation { rank: r }),
        }
    }
}
