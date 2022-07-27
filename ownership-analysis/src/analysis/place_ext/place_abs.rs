use rustc_middle::mir::{HasLocalDecls, Local};

use crate::analysis::OwnershipAnalysisCtxt;

crate::macros::newtype_index! {
    pub struct AggregateOffset {
        DEBUG_FORMAT = "{}"
    }
}

impl AggregateOffset {
    pub const ZERO: Self = AggregateOffset::from_u32(0);
}

impl std::ops::Add<AggregateOffset> for AggregateOffset {
    type Output = Self;

    fn add(self, rhs: AggregateOffset) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl std::ops::AddAssign for AggregateOffset {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

/// place abstraction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlaceAbs {
    pub base_local: Local,
    pub dereferenced: bool,
    pub(super) start: AggregateOffset,
    pub(super) end: AggregateOffset,
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
    pub fn from_local<'octxt, 'tcx, D>(
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
            start: AggregateOffset::ZERO,
            end: local_decls.local_decls()[local]
                .ty
                .ty_adt_def()
                .and_then(|adt_def| {
                    octxt
                        .program
                        .struct_topology()
                        .struct_offset(&adt_def.did())
                })
                .unwrap_or(AggregateOffset::ZERO),
        }
    }
}
