use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Field, Local, Location, PlaceRef, ProjectionElem},
    ty::TyCtxt,
};

pub trait AnalysisResults {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool>;
    fn local_result_at(
        &self,
        func: LocalDefId,
        local: Local,
        loc: Location,
        ptr_depth: usize,
    ) -> Option<bool>;
    fn field_result(&self, def_id: LocalDefId, field: Field, ptr_depth: usize) -> Option<bool>;
    fn sig_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool>;

    fn place_result<'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        def_id: LocalDefId,
        loc: Location,
        place: PlaceRef<'tcx>,
    ) -> Option<bool> {
        let body = tcx.optimized_mir(def_id);
        if !place.ty(body, tcx).ty.is_unsafe_ptr() {
            // no ownership info for non-pointer type
            return None;
        }
        assert!(place
            .projection
            .iter()
            .all(|e| matches!(e, ProjectionElem::Field(_, _) | ProjectionElem::Deref)));
        if let Some((struct_def_id, field, n_derefs)) = crate::get_struct_field(tcx, body, place) {
            return self.field_result(struct_def_id, field, n_derefs);
        } else {
            let n_derefs = place.projection.len();
            return self.local_result_at(def_id, place.local, loc, n_derefs);
        }
    }
}
