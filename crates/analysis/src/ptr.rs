use rustc_middle::ty::{AdtDef, Ty};

use crate::ownership::Precision;

pub type Measure = u32;

pub trait Measurable {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        // self.measure(ty, 0) > 0
        self.measure(ty, 0) > 0
    }

    #[inline]
    fn measure_ptr(&self, ty: Ty) -> Measure {
        let (ptr_measure, _) = abstract_ty(ty);
        ptr_measure
    }

    fn measure(&self, ty: Ty, ptr_chased: u32) -> Measure;

    fn measure_adt(&self, adt_def: AdtDef, ptr_chased: u32) -> Measure;

    fn field_offset(&self, adt_def: AdtDef, field: usize, ptr_chased: u32) -> Measure;

    // fn leaf_nodes(&self, adt_def: AdtDef, ptr_chased: u32) -> Option
    fn leaf_nodes(
        &self,
        adt_def: AdtDef,
        ptr_chased: u32,
    ) -> Option<&[((rustc_hir::def_id::DefId, usize, u32), u32)]>;

    fn max_precision(&self) -> Precision;

    /// [`determine_precision(ty, _)`] is the inverse of [`measure(ty, _)`]
    fn determine_precision(&self, ty: Ty, measure: Measure) -> Precision {
        let max_precision = self.max_precision();
        assert!(max_precision > 0);
        let mut ptr_chased = max_precision;
        while self.measure(ty, ptr_chased as u32) < measure {
            ptr_chased = ptr_chased.checked_sub(1).unwrap()
        }

        assert_eq!(self.measure(ty, ptr_chased as u32), measure);

        max_precision - ptr_chased
    }
}

impl<M: Measurable> Measurable for &M {
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
    fn max_precision(&self) -> Precision {
        (*self).max_precision()
    }

    fn leaf_nodes(
        &self,
        adt_def: AdtDef,
        ptr_chased: u32,
    ) -> Option<&[((rustc_hir::def_id::DefId, usize, u32), u32)]> {
        (*self).leaf_nodes(adt_def, ptr_chased)
    }
}

/// Abstraction of types: `&..&Adt`
#[inline]
pub fn abstract_ty(ty: Ty) -> (Measure, Option<AdtDef>) {
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
