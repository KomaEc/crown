//! Extensions to mir Place

pub mod place_abs;

use place_abs::PlaceAbs;
use rustc_middle::mir::{Place, ProjectionElem};

use super::OwnershipAnalysisCtxt;

pub trait PlaceExt<'tcx> {
    fn r#abstract<'octxt>(self, octxt: OwnershipAnalysisCtxt<'octxt, 'tcx>) -> PlaceAbs;
}

impl<'tcx> PlaceExt<'tcx> for Place<'tcx> {
    fn r#abstract<'octxt>(self, octxt: OwnershipAnalysisCtxt<'octxt, 'tcx>) -> PlaceAbs {
        let mut ans = PlaceAbs::from_local(self.local);
        for (place, projection_elem) in self.iter_projections() {
            match projection_elem {
                ProjectionElem::Deref => ans.dereferenced = true,
                ProjectionElem::Field(field, _) => {
                    todo!()
                },
                _ => continue,
            }
        }
        ans
    }
}
