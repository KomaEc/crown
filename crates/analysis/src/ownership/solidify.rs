use common::{CrateData, data_structure::vec_vec::VecVec};
use rustc_hash::FxHashMap;
use rustc_index::vec::IndexVec;

use super::{Ownership, OwnershipSchemes};
use crate::type_qualifier::flow_insensitive::{TypeQualifiers, Var};

pub type SolidifiedOwnershipSchemes = TypeQualifiers<Ownership>;

pub trait Solidifiable: for<'analysis> OwnershipSchemes<'analysis> {
    fn solidify(&self, crate_data: &CrateData) -> SolidifiedOwnershipSchemes;
}

impl<Results> Solidifiable for Results where
    Results: for<'analysis> OwnershipSchemes<'analysis>
{
    fn solidify(&self, crate_data: &CrateData) -> SolidifiedOwnershipSchemes {
        // let tcx = crate_data.tcx;
        // let mut model = IndexVec::new();
        // let mut next: Var = model.next_index();
        // let mut did_idx = FxHashMap::default();
        // did_idx.reserve(crate_data.structs.len());
        // let mut vars =
        //     VecVec::with_capacity(crate_data.structs.len(), crate_data.structs.len() * 4);

        
        todo!()
    }
}
