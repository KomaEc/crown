use std::marker::PhantomData;

use rustc_middle::ty::{Ty, TyS, TyCtxt};

pub struct LabeledTyS<'tcx, L> {
    pub ty: TyS<'tcx>,
    pub label: L
}

pub type LabeledTy<'tcx, L> = &'tcx LabeledTyS<'tcx, L>;

impl<'tcx, L> LabeledTyS<'tcx, L> {
    pub fn ty(&'tcx self) -> Ty<'tcx> {
        &self.ty
    }
}

pub struct LabeledTyCtxt<'tcx, L> {
    pub tcx: TyCtxt<'tcx>,
    _state: PhantomData<L>
}