use rustc_middle::ty::{AdtDef, Ty};

use crate::ownership::Precision;

pub type Measure = u32;

pub trait Measurable<'tcx> {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        // self.measure(ty, 0) > 0
        self.measure(ty, 0) > 0
    }

    #[inline]
    fn measure_ptr(&self, ty: Ty) -> Measure {
        let (ptr_measure, _) = decompose_ty(ty);
        ptr_measure
    }

    fn measure(&self, ty: Ty, ptr_chased: u32) -> Measure;

    fn measure_adt(&self, adt_def: AdtDef, ptr_chased: u32) -> Measure;

    fn field_offset(&self, adt_def: AdtDef, field: usize, ptr_chased: u32) -> Measure;

    fn leaf_nodes(&self, adt_def: AdtDef, ptr_chased: u32) -> &[(Ty<'tcx>, u32)];

    fn max_ptr_chased(&self) -> Precision;

    fn absolute_precision(&self, ty: Ty, measure: Measure) -> Precision;
}

impl<'tcx, M: Measurable<'tcx>> Measurable<'tcx> for &M {
    #[inline]
    fn measure(&self, ty: Ty, ptr_chased: u32) -> Measure {
        (*self).measure(ty, ptr_chased)
    }

    #[inline]
    fn measure_adt(&self, adt_def: AdtDef, ptr_chased: u32) -> Measure {
        (*self).measure_adt(adt_def, ptr_chased)
    }

    #[inline]
    fn field_offset(&self, adt_def: AdtDef, field: usize, ptr_chased: u32) -> Measure {
        (*self).field_offset(adt_def, field, ptr_chased)
    }

    #[inline]
    fn max_ptr_chased(&self) -> Precision {
        (*self).max_ptr_chased()
    }

    fn leaf_nodes(&self, adt_def: AdtDef, ptr_chased: u32) -> &[(Ty<'tcx>, u32)] {
        (*self).leaf_nodes(adt_def, ptr_chased)
    }

    fn absolute_precision(&self, ty: Ty, measure: Measure) -> Precision {
        (*self).absolute_precision(ty, measure)
    }
}

#[inline]
pub fn decompose_ty(ty: Ty) -> (Measure, Option<AdtDef>) {
    let mut ty = ty;

    let mut ptr_measure = 0;
    loop {
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }

        if let Some(ty_mut) = ty.builtin_deref(true) {
            ty = ty_mut.ty;
            ptr_measure += 1;
            continue;
        }

        break;
    }

    (ptr_measure, ty.ty_adt_def())
}
