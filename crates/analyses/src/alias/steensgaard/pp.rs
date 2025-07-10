use rustc_middle::ty::TyCtxt;
use utils::itertools::Itertools;

use crate::alias::steensgaard::{
    FieldInsensitive, InterProceduralStrategy, NopDeallocArg, Steensgaard,
};

impl<I: InterProceduralStrategy> Steensgaard<FieldInsensitive, NopDeallocArg, I> {
    pub fn pretty(&self, tcx: TyCtxt) -> String {
        let mut fns = self.fn_locals.0.did_idx.keys().copied().collect::<Vec<_>>();
        fns.sort_by_key(|did| tcx.def_path_str(*did));

        fns.into_iter()
            .map(|did| {
                self.fn_locals
                    .memory_locations(&did)
                    .iter()
                    .map(|&loc| self.pts_targets.find(self.pts[loc]))
                    .enumerate()
                    .map(|(idx, tgt)| format!("{}.{idx} -> {:?}", tcx.def_path_str(did), tgt))
                    .join("\n")
            })
            .join("\n")
    }
}
