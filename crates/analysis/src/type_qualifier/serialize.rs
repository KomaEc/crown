use std::collections::HashMap;

use common::CrateData;
use rustc_middle::mir::VarDebugInfoContents;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use super::flow_insensitive::TypeQualifiers;

#[derive(Serialize, Deserialize)]
pub struct QualifierData<Qualifier> {
    pub fn_data: HashMap<String, HashMap<String, SmallVec<[Qualifier; 2]>>>,
    pub struct_data: HashMap<String, HashMap<String, SmallVec<[Qualifier; 2]>>>,
}

impl<Qualifier> TypeQualifiers<Qualifier>
where
    Qualifier: Copy,
{
    pub fn make_data(&self, crate_data: &CrateData) -> QualifierData<Qualifier> {
        let tcx = crate_data.tcx;
        let mut fn_data = HashMap::with_capacity(crate_data.fns.len());
        let mut struct_data = HashMap::with_capacity(crate_data.structs.len());

        for &did in &crate_data.fns {
            let fn_results = self.fn_results(&did);
            let def_path_str = tcx.def_path_str(did);
            let body = tcx.optimized_mir(did);

            let user_idents = body
                .var_debug_info
                .iter()
                .filter_map(|debug_info| match debug_info.value {
                    VarDebugInfoContents::Place(place) => {
                        let local = place
                            .as_local()
                            .expect("user variable should be mapped to a local");
                        Some((local, debug_info.name))
                    }
                    _ => None,
                })
                .collect::<HashMap<_, _>>();

            let mut data = HashMap::with_capacity(user_idents.len());

            for (local, qualifiers) in
                itertools::izip!(body.local_decls.indices(), fn_results.results())
            {
                if let Some(var_name) = user_idents.get(&local) {
                    data.insert(
                        var_name.as_str().to_owned(),
                        SmallVec::from_slice(qualifiers),
                    );
                }
            }

            fn_data.insert(def_path_str, data);
        }

        for &did in &crate_data.structs {
            let def_path_str = tcx.def_path_str(did);

            let adt_def = tcx.adt_def(did);

            let mut data = HashMap::with_capacity(adt_def.all_fields().count());

            for (field_def, qualifiers) in
                itertools::izip!(adt_def.all_fields(), self.struct_results(&did))
            {
                data.insert(
                    field_def.name.as_str().to_owned(),
                    SmallVec::from_slice(qualifiers),
                );
            }

            struct_data.insert(def_path_str, data);
        }

        QualifierData {
            fn_data,
            struct_data,
        }
    }
}
