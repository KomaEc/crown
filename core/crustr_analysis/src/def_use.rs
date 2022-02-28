//! Distinguish def/use in MIR

use rustc_index::vec::IndexVec;
use rustc_middle::mir::visit::{
    MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor,
};
use rustc_middle::mir::{Body, Local, Location, Place};
use smallvec::{smallvec, SmallVec};

pub trait IsDefUse: PartialEq + Eq + Clone + Copy {
    fn defining(def_use: Self) -> bool;

    fn using(def_use: Self) -> bool;

    fn categorize(context: PlaceContext) -> Option<Self>;
}

impl IsDefUse for BorrowckDefUse {
    #[inline]
    fn defining(def_use: Self) -> bool {
        matches!(def_use, BorrowckDefUse::Def)
    }

    #[inline]
    fn using(def_use: Self) -> bool {
        matches!(def_use, BorrowckDefUse::Use)
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

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BorrowckDefUse {
    Def,
    Use,
    Drop,
}

macro_rules! make_sites_gatherer(
    ($Gatherer:ident, $def_or_use:ident) => {
        pub struct $Gatherer<'gatherer, 'tcx, DefUse>
        where DefUse: IsDefUse
        {
            body: &'gatherer Body<'tcx>,
            sites: IndexVec<Local, SmallVec<[Location; 2]>>,
            _marker: core::marker::PhantomData<*const DefUse>
        }

        impl<'gatherer, 'tcx, DefUse> $Gatherer<'gatherer, 'tcx, DefUse>
        where DefUse: IsDefUse
        {
            pub fn new(body: &'gatherer Body<'tcx>) -> Self {
                $Gatherer {
                    body,
                    sites: IndexVec::from_elem(smallvec![], &body.local_decls),
                    _marker: core::marker::PhantomData
                }
            }
            pub fn gather(mut self) -> IndexVec<Local, SmallVec<[Location; 2]>> {
                self.visit_body(self.body);
                self.sites
            }
        }

        impl<'gatherer, 'tcx, DefUse> Visitor<'tcx> for $Gatherer<'gatherer, 'tcx, DefUse>
        where DefUse: IsDefUse
        {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                location: Location
            ) {
                if DefUse::categorize(context).map_or(false, |def_use| DefUse::$def_or_use(def_use)) {
                    self.sites[place.local].push(location)
                }
            }
        }
    }
);

make_sites_gatherer!(DefSitesGatherer, defining);
make_sites_gatherer!(UseSitesGatherer, using);

pub type DefSites = IndexVec<Local, SmallVec<[Location; 2]>>;
pub type UseSites = IndexVec<Local, SmallVec<[Location; 2]>>;
