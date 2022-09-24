use rustc_middle::ty::{AdtDef, Ty};

pub type Measure = u32;

pub trait Measurable {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        self.measure(ty) > 0
    }

    #[inline]
    fn measure(&self, ty: Ty) -> Measure {
        // self.measure_with_threshold(ty, Level(0))
        let (ptr_measure, adt) = abstract_ty(ty);
        if ptr_measure > 0 {
            1
        } else {
            adt.map(|adt| self.measure_adt(adt)).unwrap_or_default()
        }
    }

    fn measure_adt(&self, adt_def: AdtDef) -> Measure;

    // /// Note: blocked by adt
    // #[inline]
    // fn measure_with_threshold(&self, ty: Ty, threshold: Level) -> Measure {
    //     let mut ty = ty;

    //     while let Some(inner_ty) = ty.builtin_index() {
    //         ty = inner_ty
    //     }

    //     if let Some(inner_ty_mut) = ty.builtin_deref(true) {
    //         if threshold.as_u8() == 0 {
    //             return 1;
    //         }
    //         ty = inner_ty_mut.ty;
    //         return self.measure_with_threshold(ty, threshold - 1) + 1;
    //     }

    //     if let TyKind::Adt(adt_def, _) = ty.kind() {
    //         self.measure_adt(adt_def)
    //     } else {
    //         0
    //     }
    // }
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

// /// Level of pointer chasing
// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
// pub struct Level(u8);

// impl Level {
//     #[inline]
//     pub fn as_u8(self) -> u8 {
//         self.0
//     }
// }

// impl std::ops::Add<u8> for Level {
//     type Output = Self;

//     fn add(mut self, rhs: u8) -> Self::Output {
//         self.0 += rhs;
//         self
//     }
// }

// impl std::ops::Sub<u8> for Level {
//     type Output = Self;

//     fn sub(mut self, rhs: u8) -> Self::Output {
//         self.0 -= rhs;
//         self
//     }
// }
