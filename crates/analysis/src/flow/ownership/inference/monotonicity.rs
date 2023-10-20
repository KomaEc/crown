use rustc_middle::ty::{Ty, TyCtxt, TyKind};

use crate::flow::ownership::{
    constraint::{Constraint, Database, OwnershipToken, StorageMode},
    InterproceduralView,
};

impl<'analysis, Mode, DB> InterproceduralView<'analysis, Mode, DB>
where
    Mode: StorageMode,
    DB: Database<Mode>,
{
    pub fn enforce_monotonicity<'tcx>(
        &mut self,
        mut ty: Ty<'tcx>,
        mut current_token: OwnershipToken,
        mut parent_token: Option<OwnershipToken>,
        mut k_limit: usize,
        tcx: TyCtxt<'tcx>,
    ) {
        if k_limit == 0 {
            return;
        }

        loop {
            if let Some(inner_ty) = ty.builtin_index() {
                ty = inner_ty;
                continue;
            }
            if let Some(ty_mut) = ty.builtin_deref(true) {
                if let Some(parent_token) = parent_token {
                    self.database.add(
                        Constraint::LessEqual {
                            x: current_token,
                            y: parent_token,
                        },
                        &mut self.storage,
                    )
                }

                parent_token = Some(current_token);
                current_token += 1;
                k_limit -= 1;
                if k_limit == 0 {
                    return;
                }
                ty = ty_mut.ty;
                continue;
            }
            break;
        }

        if self.access_paths.size_of(k_limit, ty) > 0 {
            let TyKind::Adt(adt_def, subst) = ty.kind() else {
                unreachable!()
            };
            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, subst);
                self.enforce_monotonicity(field_ty, current_token, parent_token, k_limit, tcx)
            }
        }
    }
}
