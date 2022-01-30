use smallvec::SmallVec;
use std::fmt::Display;

use rustc_hir::def_id::DefId;
use rustc_middle::mir::Local;
use rustc_target::abi::VariantIdx;

pub enum Position {
    Field {
        adt_def: DefId,
        variant_idx: VariantIdx,
    },
    Local {
        body_def: DefId,
        local: Local,
        ssa_idx: SSAIdx,
    },
}

rustc_index::newtype_index! {
    pub struct SSAIdx {
        DEBUG_FORMAT = "_{}"
    }
}

rustc_index::newtype_index! {
    pub struct OwnershipVar {
        DEBUG_FORMAT = "ρ_({})"
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Constraint {
    /// p = q ~~> rho_p' + rho_q' = rho_q
    MaybeOwnershipTransfer {
        src_before: OwnershipVar,
        src_after: OwnershipVar,
        dest_after: OwnershipVar,
    },

    /// rho_p = 1
    AssertOwned(OwnershipVar),

    /// rho_p = 0
    AssertTransient(OwnershipVar),

    /// pairwise equality
    AssertEqual(SmallVec<[OwnershipVar; 2]>),
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaybeOwnershipTransfer {
                src_before,
                src_after,
                dest_after,
            } => f.write_fmt(format_args!(
                "{:?} + {:?} = {:?}",
                dest_after, src_after, src_before
            )),
            Self::AssertOwned(o) => f.write_fmt(format_args!("{:?} = 1", o)),
            Self::AssertTransient(o) => f.write_fmt(format_args!("{:?} = 0", o)),
            Self::AssertEqual(os) => {
                if os.len() == 2 {
                    f.write_fmt(format_args!("{:?} = {:?}", os[0], os[1]))
                } else {
                    f.write_str("PairwiseEqual(")?;
                    f.debug_list().entries(os).finish()?;
                    f.write_str(")")
                }
            }
        }
    }
}
