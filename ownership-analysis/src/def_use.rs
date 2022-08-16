//! Distinguish def/use in MIR

use rustc_index::vec::IndexVec;
use rustc_middle::mir::visit::{
    MutatingUseContext, NonMutatingUseContext, NonUseContext, PlaceContext, Visitor,
};
use rustc_middle::mir::{Body, Local, LocalDecls, Location, Place};
use rustc_middle::ty::TyCtxt;
use smallvec::{smallvec, SmallVec};

use crate::ty_ext::TyExt;

pub trait IsDefUse: PartialEq + Eq + Clone + Copy {
    fn defining(self) -> bool;

    fn using(self) -> bool;

    /// This is dedicated for catch up cases of `place`, return, index
    fn categorise_for_local<'tcx>(
        local: Local,
        body: &LocalDecls<'tcx>,
        context: PlaceContext,
    ) -> Option<Self>;

    fn categorise_for_place<'tcx>(
        place: &Place<'tcx>,
        context: PlaceContext,
        body: &Body<'tcx>,
        _tcx: TyCtxt<'tcx>,
    ) -> Option<Self> {
        let mut context = context;

        if !place.projection.is_empty() {
            if context.is_use() {
                // ^ Only change the context if it is a real use, not a "use" in debuginfo.
                context = if context.is_mutating_use() {
                    PlaceContext::MutatingUse(MutatingUseContext::Projection)
                } else {
                    PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection)
                };
            }
        }

        Self::categorise_for_local(place.local, &body.local_decls, context)
    }

    fn gather<'tcx>(body: &Body<'tcx>, _tcx: TyCtxt<'tcx>) -> DefSites {
        DefaultDefSitesGatherer::<Self>::gather(body)
    }
}

impl DefaultDefUse {
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
            PlaceContext::NonUse(NonUseContext::StorageDead) => Some(DefaultDefUse::Def),

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
                Some(DefaultDefUse::Use),


            // These are uses that occur in a DROP (a MIR drop, not a
            // call to `std::mem::drop()`). For the purposes of NLL,
            // uses in drop are special because `#[may_dangle]`
            // attributes can affect whether lifetimes must be live.

            PlaceContext::MutatingUse(MutatingUseContext::Drop) =>
                Some(DefaultDefUse::Drop),

            // Debug info is neither def nor use.
            PlaceContext::NonUse(NonUseContext::VarDebugInfo) => None,
            _ => unreachable!()
        }
    }
}

impl IsDefUse for DefaultDefUse {
    #[inline]
    fn defining(self) -> bool {
        matches!(self, DefaultDefUse::Def)
    }

    #[inline]
    fn using(self) -> bool {
        matches!(self, DefaultDefUse::Use)
    }

    fn categorise_for_local<'tcx>(
        _local: Local,
        _body: &LocalDecls<'tcx>,
        context: PlaceContext,
    ) -> Option<Self> {
        Self::categorize(context)
    }
}

impl OwnershipAnalysisDefUse {
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
            PlaceContext::NonUse(NonUseContext::StorageDead) => Some(OwnershipAnalysisDefUse::Def),

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
                Some(OwnershipAnalysisDefUse::Use),


            // These are uses that occur in a DROP (a MIR drop, not a
            // call to `std::mem::drop()`). For the purposes of NLL,
            // uses in drop are special because `#[may_dangle]`
            // attributes can affect whether lifetimes must be live.

            PlaceContext::MutatingUse(MutatingUseContext::Drop) =>
                None,

            // Debug info is neither def nor use.
            PlaceContext::NonUse(NonUseContext::VarDebugInfo) => None,
            _ => unreachable!()
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

    fn categorise_for_local<'tcx>(
        local: Local,
        body: &LocalDecls<'tcx>,
        context: PlaceContext,
    ) -> Option<Self> {
        if body[local].ty.is_ptr_but_not_fn_ptr()
            && matches!(
                context,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Copy)
                    | PlaceContext::NonMutatingUse(NonMutatingUseContext::Move)
            )
        {
            Some(OwnershipAnalysisDefUse::Def)
        } else {
            Self::categorize(context)
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DefaultDefUse {
    Def,
    Use,
    Drop,
}

pub type FatThinAnalysisDefUse = DefaultDefUse;
pub type MutabilityAnalysisDefUse = DefaultDefUse;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum OwnershipAnalysisDefUse {
    Def,
    Use,
}

pub struct DefaultDefSitesGatherer<'me, 'tcx, DefUse>
where
    DefUse: IsDefUse,
{
    body: &'me LocalDecls<'tcx>,
    sites: IndexVec<Local, SmallVec<[Location; 2]>>,
    _marker: core::marker::PhantomData<*const DefUse>,
}

impl<'me, 'tcx, DefUse> DefaultDefSitesGatherer<'me, 'tcx, DefUse>
where
    DefUse: IsDefUse,
{
    pub fn gather(body: &Body<'tcx>) -> IndexVec<Local, SmallVec<[Location; 2]>> {
        let mut gatherer: DefaultDefSitesGatherer<DefUse> = DefaultDefSitesGatherer {
            body: &body.local_decls,
            sites: IndexVec::from_elem(smallvec![], &body.local_decls),
            _marker: core::marker::PhantomData,
        };
        gatherer.visit_body(body);
        gatherer.sites
    }
}

impl<'me, 'tcx, DefUse> Visitor<'tcx> for DefaultDefSitesGatherer<'me, 'tcx, DefUse>
where
    DefUse: IsDefUse,
{
    /// we visit local because of RETURN clauses!!
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if DefUse::categorise_for_local(local, self.body, context)
            .map_or(false, |def_use| DefUse::defining(def_use))
        {
            self.sites[local].push(location)
        }
    }
}

/// By using this data structure, we are assuming that a local is
/// defined (or used!!) at most once in a location.
/// For instance, `f(s.f, s.g)` is assumed to be lowered to something
/// like `tmp1 = s.f; tmp2 = s.g; f(tmp1, tmp2)`
/// This holds for all example codes that we've seen so far.
pub type DefSites = IndexVec<Local, SmallVec<[Location; 2]>>;
