use std::num::NonZeroU32;

use rustc_middle::{
    mir::{LocalDecl, LocalInfo},
    ty::{AdtDef, Ty, TyKind},
};

pub type Measure = u32;

pub trait Measurable {
    #[inline]
    fn contains_ptr(&self, ty: Ty) -> bool {
        self.measure(ty) > 0
    }

    #[inline]
    fn measure(&self, ty: Ty) -> Measure {
        self.measure_with_threshold(ty, Level(0))
    }

    fn measure_adt(&self, adt_def: &AdtDef) -> Measure;

    /// Note: blocked by adt
    #[inline]
    fn measure_with_threshold(&self, ty: Ty, threshold: Level) -> Measure {
        let mut ty = ty;

        while let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty
        }

        if let Some(inner_ty_mut) = ty.builtin_deref(true) {
            if threshold.as_u8() == 0 {
                return 1;
            }
            ty = inner_ty_mut.ty;
            return self.measure_with_threshold(ty, threshold - 1) + 1;
        }

        if let TyKind::Adt(adt_def, _) = ty.kind() {
            self.measure_adt(adt_def)
        } else {
            0
        }
    }
}

/// test whether a local might be owning
#[inline]
pub fn try_measure_local(
    local_decl: &LocalDecl,
    measurable: impl Measurable,
) -> Option<NonZeroU32> {
    let ty = local_decl.ty;
    let measure = measurable.measure(ty);
    (!matches!(local_decl.local_info, Some(box LocalInfo::DerefTemp)))
        .then(|| NonZeroU32::new(measure))
        .flatten()
}

/// Level of pointer chasing
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Level(u8);

impl Level {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

impl std::ops::Add<u8> for Level {
    type Output = Self;

    fn add(mut self, rhs: u8) -> Self::Output {
        self.0 += rhs;
        self
    }
}

impl std::ops::Sub<u8> for Level {
    type Output = Self;

    fn sub(mut self, rhs: u8) -> Self::Output {
        self.0 -= rhs;
        self
    }
}
