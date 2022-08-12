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
        octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>,
    ) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>;
}

impl<'tcx> PlaceExt<'tcx> for Place<'tcx> {
    fn r#abstract<'octxt, D>(
        self,
        local_decls: &D,
        octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>,
    ) -> Option<PlaceAbs>
    where
        D: HasLocalDecls<'tcx>,
    {
        let mut ans = PlaceAbs::from_local(self.local, local_decls, octxt);
        for (place, projection_elem) in self.iter_projections() {
            match projection_elem {
                ProjectionElem::Deref => ans.dereferenced = true,
                ProjectionElem::Field(field, _) => {
                    let place_ty = place.ty(local_decls, octxt.program.tcx);
                    let ty = place_ty.ty;
                    if matches!(ty.kind(), TyKind::Tuple(..)) {
                        return None;
                    }
                    let TyKind::Adt(adt_def, _) = ty.kind() else { unreachable!("{}", place_ty.ty) };
                    let offsets = octxt
                        .program
                        .struct_topology()
                        .field_offsets(&adt_def.did())?;
                    if offsets.len() == 1 {
                        return None;
                    }
                    let (&[field_offset_start, field_offset_end], _) =
                        offsets[field.as_usize()..field.as_usize() + 2].split_array_ref();
                    ans.end = ans.start + field_offset_end;
                    ans.start = ans.start + field_offset_start;
                }
                _ => continue,
            }
        }
        (ans.start == ans.end)
            .then(|| None)
            .unwrap_or_else(|| Some(ans))
    }
}
