#![feature(rustc_private)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]
#![feature(associated_type_defaults)]
#![feature(split_array)]
#![feature(array_windows)]
#![feature(allocator_api)]

extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;

pub mod captures;
pub mod data_structure;
pub mod discretization;
pub mod macros;
pub mod rewrite;
pub mod struct_dependency;
pub mod test;

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

pub struct CrateData<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub fns: Vec<DefId>,
    pub structs: Vec<DefId>,
}

impl<'tcx> CrateData<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, fns: Vec<DefId>, structs: Vec<DefId>) -> Self {
        Self { tcx, fns, structs }
    }
}
