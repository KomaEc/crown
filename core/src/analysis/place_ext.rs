//! Extensions to mir Place

pub mod place_abs;

use place_abs::PlaceAbs;
use rustc_middle::{
    mir::{HasLocalDecls, Place, ProjectionElem},
    ty::TyKind,
};

use super::OwnershipAnalysisCtxt;

pub trait PlaceExt<'tcx> {
    fn r#abstract<'octxt, D>(
        self,
        local_decls: &D,
        octxt: OwnershipAnalysisCtxt<'octxt, 'tcx>,
    ) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>;
}

impl<'tcx> PlaceExt<'tcx> for Place<'tcx> {
    fn r#abstract<'octxt, D>(
        self,
        local_decls: &D,
        octxt: OwnershipAnalysisCtxt<'octxt, 'tcx>,
    ) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>,
    {
        let mut ans = PlaceAbs::from_local(self.local);
        for (place, projection_elem) in self.iter_projections() {
            match projection_elem {
                ProjectionElem::Deref => ans.dereferenced = true,
                ProjectionElem::Field(field, _) => {
                    let place_ty = place.ty(local_decls, octxt.program.tcx);
                    let TyKind::Adt(adt_def, _) = place_ty.ty.kind() else { unreachable!("impossible") };
                    assert!(adt_def.is_struct());
                    ans.offset += octxt
                        .program
                        .struct_topology()
                        .field_offsets(&adt_def.did())?[field.as_usize()]
                }
                _ => continue,
            }
        }
        Some(ans)
    }
}
