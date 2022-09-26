use rustc_middle::ty::{AdtDef, Ty};

pub type Measure = u32;

pub trait Measurable {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        // self.measure(ty) > 0
        self.measure(ty, 0) > 0
    }

    #[inline]
    fn measure_ptr(&self, ty: Ty) -> Measure {
        let (ptr_measure, _) = abstract_ty(ty);
        ptr_measure
    }

    #[inline]
    fn measure(&self, ty: Ty, allowed_derefs: u32) -> Measure {
        let (ptr_measure, maybe_adt) = abstract_ty(ty);
        let adt_measure = maybe_adt.map(|adt_def| self.measure_adt(adt_def));
        if ptr_measure > allowed_derefs {
            allowed_derefs + 1
        } else {
            ptr_measure + adt_measure.unwrap_or_default()
        }
    }

    fn measure_adt(&self, adt_def: AdtDef) -> Measure;
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
