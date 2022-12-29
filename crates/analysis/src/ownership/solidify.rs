use common::{
    data_structure::vec_vec::VecVec,
    discretization::{self, Discretization},
    CrateData,
};
use rustc_hash::FxHashMap;
use rustc_index::vec::IndexVec;

use super::{whole_program::WholeProgramResults, Ownership};
use crate::type_qualifier::flow_insensitive::{TypeQualifiers, Var};

pub type SolidifiedOwnershipSchemes = TypeQualifiers<Ownership>;

impl<'tcx> WholeProgramResults<'tcx> {
    fn solidify(&self, crate_data: &CrateData) -> SolidifiedOwnershipSchemes {
        let tcx = crate_data.tcx;
        let mut model = IndexVec::new();
        let mut next: Var = model.next_index();
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut vars =
            VecVec::with_capacity(crate_data.structs.len(), crate_data.structs.len() * 4);

        for (idx, r#struct) in crate_data.structs.iter().enumerate() {
            did_idx.insert(*r#struct, idx);

            let fields_ownership = self.fields(r#struct);
            for ownership in fields_ownership {
                model.extend(ownership);
                vars.push_inner(next);
                next = next + ownership.len();
                assert_eq!(model.next_index(), next);
            }
            vars.push_inner(next);
            vars.push();
        }
        let vars = vars.done();
        let struct_fields = discretization::StructFields(Discretization {
            did_idx,
            contents: vars,
        });

        // let mut did_idx = FxHashMap::default();
        // did_idx.reserve(crate_data.fns.len());
        // let mut vars = VecVec::with_capacity(crate_data.fns.len(), crate_data.fns.len() * 15);
        // for (idx, r#fn) in crate_data.fns.iter().enumerate() {
        //     did_idx.insert(*r#fn, idx);
        // }

        todo!()
    }
}
