use rustc_middle::mir::{HasLocalDecls, Local};

use crate::analysis::OwnershipAnalysisCtxt;

orc_common::macros::newtype_index! {
    pub(crate) struct Offset {
        DEBUG_FORMAT = "{}"
    }
}

impl Offset {
    pub(crate) const ZERO: Self = Offset::from_u32(0);
}

impl std::ops::Add<Offset> for Offset {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl std::ops::AddAssign for Offset {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

/// place abstraction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct PlaceAbs {
    pub(crate) base_local: Local,
    pub(crate) dereferenced: bool,
    pub(crate) start: Offset,
    pub(crate) end: Offset,
}

impl std::fmt::Display for PlaceAbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start >= self.end {
            // if self.dereferenced {
            //     f.write_str("*")?
            // }
            // f.write_fmt(format_args!("{:?}", self.base_local))
            unreachable!("{:?} >= {:?}", self.start, self.end)
        } else {
            if self.dereferenced {
                f.write_str("(*")?
            }
            f.write_fmt(format_args!("{:?}", self.base_local))?;
            if self.dereferenced {
                f.write_str(")")?
            }
            if self.start + 1 == self.end {
                f.write_fmt(format_args!(".{:?}", self.start))
            } else {
                f.write_fmt(format_args!(".[{:?}~{:?}]", self.start, self.end))
            }
        }
    }
}

impl PlaceAbs {
    pub(crate) fn from_local<'octxt, 'tcx, D>(
        local: Local,
        local_decls: &D,
        octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>,
    ) -> Self
    where
        D: HasLocalDecls<'tcx>,
    {
        PlaceAbs {
            base_local: local,
            dereferenced: false,
            start: Offset::ZERO,
            end: local_decls.local_decls()[local]
                .ty
                .ty_adt_def()
                .and_then(|adt_def| {
                    adt_def.is_struct().then(|| {
                        octxt
                            .program
                            .struct_topology()
                            .struct_offset(&adt_def.did())
                    })
                })
                .unwrap_or(Offset::ZERO),
        }
    }
}
