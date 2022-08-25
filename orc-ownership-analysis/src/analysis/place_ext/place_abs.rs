use rustc_middle::mir::{HasLocalDecls, Local};

use crate::analysis::OwnershipAnalysisCtxt;

orc_common::macros::newtype_index! {
    pub(crate) struct AggregateOffset {
        DEBUG_FORMAT = "{}"
    }
}

impl AggregateOffset {
    pub(crate) const ZERO: Self = AggregateOffset::from_u32(0);
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
pub(crate) struct PlaceAbs {
    pub(crate) base_local: Local,
    pub(crate) dereferenced: bool,
    pub(crate) start: AggregateOffset,
    pub(crate) end: AggregateOffset,
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
