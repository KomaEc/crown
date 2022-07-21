//! Extensions to mir Place

pub mod place_abs;

use place_abs::PlaceAbs;
use rustc_middle::mir::{Place, ProjectionElem};

pub trait PlaceExt {
    fn r#abstract(self) -> PlaceAbs;
}

impl<'tcx> PlaceExt for Place<'tcx> {
    fn r#abstract(self) -> PlaceAbs {
        let mut ans = PlaceAbs {
            base_local: self.local,
            num_derefs: 0,
        };
        for projection_elem in self.projection {
            let ProjectionElem::Deref = projection_elem else { continue };
            ans.num_derefs += 1;
        }
        ans
    }
}
