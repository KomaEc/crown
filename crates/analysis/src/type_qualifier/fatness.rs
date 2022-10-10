use rustc_middle::{
    mir::{HasLocalDecls, Location, Place, Rvalue, Terminator},
    ty::TyCtxt,
};

use super::framework::{BooleanLatticeSystem, FnLocalsVars, Infer, StructFieldsVars, Var};

/// [`Arr`] ⊑ [`Ptr`]
pub enum FatnessLattice {
    Arr,
    Ptr,
}

pub struct Fatness;

impl Infer for Fatness {
    type C = BooleanLatticeSystem;

    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::C,
        tcx: TyCtxt<'tcx>,
    ) {
        todo!()
    }

    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::C,
        tcx: TyCtxt<'tcx>,
    ) {
        todo!()
    }
}
