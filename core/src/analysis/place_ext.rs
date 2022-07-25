//! Extensions to mir Place

pub mod place_abs;

use place_abs::PlaceAbs;
use rustc_middle::mir::{Place, ProjectionElem};

pub trait PlaceExt {
    fn r#abstract(self) -> PlaceAbs;
}

impl<'tcx> PlaceExt for Place<'tcx> {
    fn r#abstract(self) -> PlaceAbs {
        let mut ans = PlaceAbs::from_local(self.local);
        for projection_elem in self.projection {
            match projection_elem {
                ProjectionElem::Deref => ans.dereferenced = true,
                ProjectionElem::Field(_, _) => todo!(),
                _ => continue,
            }
        }
        ans
    }
}
