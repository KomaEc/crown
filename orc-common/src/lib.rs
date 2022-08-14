#![feature(rustc_private)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]

extern crate rustc_hir;
extern crate rustc_hash;
extern crate rustc_index;
extern crate rustc_middle;

pub mod macros;
pub mod two_level_discretization;

use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::{
    mir::{Body, Field, Local, Location, PlaceRef, ProjectionElem},
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
        if let Some((struct_def_id, field, n_derefs)) = get_struct_field(tcx, body, place) {
            return self.field_result(struct_def_id, field, n_derefs);
        } else {
            let n_derefs = place.projection.len();
            return self.local_result_at(def_id, place.local, loc, n_derefs);
        }
    }
}

pub fn get_struct_field<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: PlaceRef<'tcx>,
) -> Option<(LocalDefId, Field, usize)> {
    let field = place
        .iter_projections()
        .rev()
        .enumerate()
        .find(|(_i, (_base, elem))| matches!(elem, ProjectionElem::Field(_, _)));
    if let Some((n_derefs, (base, ProjectionElem::Field(field, _ty)))) = field {
        let struct_def_id = base
            .ty(body, tcx)
            .ty
            .ty_adt_def()
            .unwrap()
            .did()
            .as_local()
            .unwrap();
        Some((struct_def_id, field, n_derefs))
    } else {
        None
    }
}

pub trait OrcInput<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx>;
    fn functions(&self) -> &[DefId];
    fn structs(&self) -> &[DefId];
    fn into_trivial(self) -> (TyCtxt<'tcx>, Vec<DefId>, Vec<DefId>);
}

impl<'tcx> OrcInput<'tcx> for (TyCtxt<'tcx>, Vec<DefId>, Vec<DefId>) {
    #[inline]
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.0
    }

    #[inline]
    fn functions(&self) -> &[DefId] {
        &self.1[..]
    }

    #[inline]
    fn structs(&self) -> &[DefId] {
        &self.2[..]
    }

    #[inline]
    fn into_trivial(self) -> (TyCtxt<'tcx>, Vec<DefId>, Vec<DefId>) {
        self
    }
}

impl<'tcx, Input: OrcInput<'tcx>> OrcInput<'tcx> for &Input {
    fn tcx(&self) -> TyCtxt<'tcx> {
        (*self).tcx()
    }

    fn functions(&self) -> &[DefId] {
        (*self).functions()
    }

    fn structs(&self) -> &[DefId] {
        (*self).functions()
    }

    fn into_trivial(self) -> (TyCtxt<'tcx>, Vec<DefId>, Vec<DefId>) {
        (
            self.tcx(),
            self.functions().iter().map(|&did| did).collect(),
            self.structs().iter().map(|&did| did).collect(),
        )
    }
}
