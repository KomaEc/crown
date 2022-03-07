//! Distinguish def/use in MIR

use rustc_index::vec::IndexVec;
use rustc_middle::mir::visit::{
    MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor,
};
use rustc_middle::mir::{Body, Local, Location, Place};
use rustc_middle::ty::TyCtxt;
use smallvec::{smallvec, SmallVec};

pub trait IsDefUse: PartialEq + Eq + Clone + Copy {
    fn defining(self) -> bool;

    fn using(self) -> bool;

    fn categorize_finely<'tcx>(
        _local: Local,
        _tcx: TyCtxt<'tcx>,
        context: PlaceContext,
    ) -> Option<Self> {
        Self::categorize(context)
    }

    fn categorize(context: PlaceContext) -> Option<Self>;
}

impl IsDefUse for BorrowckDefUse {
    #[inline]
    fn defining(self) -> bool {
        matches!(self, BorrowckDefUse::Def)
    }

    #[inline]
    fn using(self) -> bool {
        matches!(self, BorrowckDefUse::Use)
    }

    #[inline]
    fn categorize(context: PlaceContext) -> Option<Self> {
        match context {

            PlaceContext::MutatingUse(MutatingUseContext::Store) |

            // We let Call define the result in both the success and
            // unwind cases. This is not really correct, however it
            // does not seem to be observable due to the way that we
            // generate MIR. To do things properly, we would apply
            // the def in call only to the input from the success
            // path and not the unwind path. -nmatsakis
            PlaceContext::MutatingUse(MutatingUseContext::Call) |
            PlaceContext::MutatingUse(MutatingUseContext::AsmOutput) |
            PlaceContext::MutatingUse(MutatingUseContext::Yield) |

            // Storage live and storage dead aren't proper defines, but we can ignore
            // values that come before them.
            PlaceContext::NonUse(NonUseContext::StorageLive) |
            PlaceContext::NonUse(NonUseContext::StorageDead) => Some(BorrowckDefUse::Def),

            // These are uses that occur *outside* of a drop. For the
            // purposes of NLL, these are special in that **all** the
            // lifetimes appearing in the variable must be live for each regular use.

            PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) |
            PlaceContext::MutatingUse(MutatingUseContext::Projection) |

            // Borrows only consider their local used at the point of the borrow.
            // This won't affect the results since we use this analysis for generators
            // and we only care about the result at suspension points. Borrows cannot
            // cross suspension points so this behavior is unproblematic.
            PlaceContext::MutatingUse(MutatingUseContext::Borrow) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::SharedBorrow) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::ShallowBorrow) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::UniqueBorrow) |

            PlaceContext::MutatingUse(MutatingUseContext::AddressOf) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::AddressOf) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Copy) |
            PlaceContext::NonMutatingUse(NonMutatingUseContext::Move) |
            PlaceContext::NonUse(NonUseContext::AscribeUserTy) |
            PlaceContext::MutatingUse(MutatingUseContext::Retag) =>
                Some(BorrowckDefUse::Use),


            // These are uses that occur in a DROP (a MIR drop, not a
            // call to `std::mem::drop()`). For the purposes of NLL,
            // uses in drop are special because `#[may_dangle]`
            // attributes can affect whether lifetimes must be live.

            PlaceContext::MutatingUse(MutatingUseContext::Drop) =>
                Some(BorrowckDefUse::Drop),

            // Debug info is neither def nor use.
            PlaceContext::NonUse(NonUseContext::VarDebugInfo) => None,
        }
    }
}

impl IsDefUse for OwnershipAnalysisDefUse {
    fn defining(self) -> bool {
        matches!(self, OwnershipAnalysisDefUse::Def)
    }

    fn using(self) -> bool {
        matches!(self, OwnershipAnalysisDefUse::Use)
    }

    fn categorize_finely<'tcx>(local: Local, tcx: TyCtxt<'tcx>, context: PlaceContext) -> Option<Self> {
        todo!()
    }

    fn categorize(context: PlaceContext) -> Option<Self> {
        todo!()
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BorrowckDefUse {
    Def,
    Use,
    Drop,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum OwnershipAnalysisDefUse {
    Def,
    Use,
    Other
}

macro_rules! make_sites_gatherer(
    ($Gatherer:ident, $def_or_use:ident) => {
        pub struct $Gatherer<'tcx, DefUse>
        where DefUse: IsDefUse
        {
            tcx: TyCtxt<'tcx>,
            sites: IndexVec<Local, SmallVec<[Location; 2]>>,
            _marker: core::marker::PhantomData<*const DefUse>
        }

        impl<'tcx, DefUse> $Gatherer<'tcx, DefUse>
        where DefUse: IsDefUse
        {
            pub fn gather(tcx: TyCtxt<'tcx>, body: &Body<'tcx>) -> IndexVec<Local, SmallVec<[Location; 2]>>{
                let mut gatherer: $Gatherer<'_, DefUse> = 
                    $Gatherer {
                        tcx,
                        sites: IndexVec::from_elem(smallvec![], &body.local_decls),
                        _marker: core::marker::PhantomData
                    };
                gatherer.visit_body(body);
                gatherer.sites
            }
        }

        impl<'tcx, DefUse> Visitor<'tcx> for $Gatherer<'tcx, DefUse>
        where DefUse: IsDefUse
        {
            fn visit_local(
                &mut self,
                &local: &Local,
                context: PlaceContext,
                location: Location
            ) {
                if DefUse::categorize_finely(local, self.tcx, context).map_or(false, |def_use| DefUse::$def_or_use(def_use)) {
                    self.sites[local].push(location)
                }
            }
        }
    }
);

make_sites_gatherer!(DefSitesGatherer, defining);
make_sites_gatherer!(UseSitesGatherer, using);

pub type DefSites = IndexVec<Local, SmallVec<[Location; 2]>>;
pub type UseSites = IndexVec<Local, SmallVec<[Location; 2]>>;
