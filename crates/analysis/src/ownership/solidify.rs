use super::{Ownership, OwnershipSchemes};
use crate::type_qualifier::flow_insensitive::AnalysisResult;

pub type SolidifiedOwnershipSchemes = AnalysisResult<Ownership>;

pub trait Solidifiable: for<'analysis> OwnershipSchemes<'analysis> {
    fn solidify(&self) -> SolidifiedOwnershipSchemes;
}

impl<Results> Solidifiable for Results where
    Results: for<'analysis> OwnershipSchemes<'analysis>
{
    fn solidify(&self) -> SolidifiedOwnershipSchemes {
        todo!()
    }
}
