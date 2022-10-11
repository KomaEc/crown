use rustc_middle::{
    mir::{HasLocalDecls, Location, Place, Rvalue, Terminator},
    ty::TyCtxt,
};

use super::{
    boolean_system::BooleanSystem, BooleanLattice, FnLocalsVars, Infer, Lattice, StructFieldsVars,
    Var,
};

#[derive(Clone, Copy, PartialEq, Eq)]
/// [`Arr`] ⊑ [`Ptr`]
pub enum Fatness {
    Arr,
    Ptr,
}

impl From<Fatness> for bool {
    fn from(fatness: Fatness) -> Self {
        fatness == Fatness::Ptr
    }
}

impl From<bool> for Fatness {
    fn from(b: bool) -> Self {
        if b {
            Fatness::Ptr
        } else {
            Fatness::Arr
        }
    }
}

impl Lattice for Fatness {
    const BOTTOM: Self = Self::Arr;

    const TOP: Self = Self::Ptr;
}

impl BooleanLattice for Fatness {}

pub struct FatnessAnalysis;

impl Infer for FatnessAnalysis {
    type L = BooleanSystem<Fatness>;

    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::L,
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
        database: &mut <Self as Infer>::L,
        tcx: TyCtxt<'tcx>,
    ) {
        todo!()
    }
}
