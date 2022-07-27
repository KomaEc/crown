//! Extensions to mir Place

pub mod place_abs;

use place_abs::PlaceAbs;
use rustc_middle::{
    mir::{HasLocalDecls, Place, ProjectionElem},
    ty::TyKind,
};

use crate::Program;

pub trait PlaceExt<'tcx> {
    fn r#abstract<'octxt, D>(self, local_decls: &D, program: &Program<'tcx>) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>;
}

impl<'tcx> PlaceExt<'tcx> for Place<'tcx> {
    fn r#abstract<'octxt, D>(self, local_decls: &D, program: &Program<'tcx>) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>,
    {
        let mut ans = PlaceAbs::from_local(self.local);
        for (place, projection_elem) in self.iter_projections() {
            match projection_elem {
                ProjectionElem::Deref => ans.dereferenced = true,
                ProjectionElem::Field(field, _) => {
                    let place_ty = place.ty(local_decls, program.tcx);
                    let ty = place_ty.ty;
                    if matches!(ty.kind(), TyKind::Tuple(..)) {
                        return None;
                    }
                    let TyKind::Adt(adt_def, _) = ty.kind() else { unreachable!("{}", place_ty.ty) };
                    ans.offset +=
                        program.struct_topology().field_offsets(&adt_def.did())?[field.as_usize()]
                }
                _ => continue,
            }
        }
        Some(ans)
    }
}
