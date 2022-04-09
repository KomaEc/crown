use rustc_middle::mir::VarDebugInfoContents;

use super::UnusedPointerDeclForBody;

impl<'updb, 'upd, 'tcx> UnusedPointerDeclForBody<'updb, 'upd, 'tcx> {
    pub fn initiate_tracked(mut self) -> Self {
        let all_user_vars = self.body.var_debug_info.iter().filter_map(|var| {
            if let VarDebugInfoContents::Place(place) = var.value {
                let ty = place.ty(&self.body.local_decls, self.tcx).ty;
                if ty.is_ptr_of_concerned()  {
                    return Some(var);
                }
            } else {
                tracing::warn!("Unused pointer decl: ignoring constant!");
            }
            None
        });

        for var_p in all_user_vars {
            if let VarDebugInfoContents::Place(place) = var_p.value {
                self.user_vars.push((place.as_ref(), &var_p))
            }
        }

        self.user_vars.sort_by_key(|(place_ref, _)| *place_ref);

        self
    }
}
