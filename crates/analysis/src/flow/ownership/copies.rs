//! Copy variables are those compiler generated temporaries that have extremely short life range:
//! 1. it is used only once after its unique definition.
//! 2. deref copies

use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    Body, Local, Location, Place, Rvalue, Statement, StatementKind,
};

pub type Copies = BitSet<Local>;

pub struct DerefCopiesCollector<'this>(pub &'this mut BitSet<Local>);

pub fn collect_copies(body: &Body) -> Copies {
    let mut deref_copies = BitSet::new_empty(body.local_decls.len());
    DerefCopiesCollector(&mut deref_copies).visit_body(body);

    let mut unique_def = IndexVec::from_elem_n(None, body.local_decls.len());
    UniqueDef(&mut unique_def).visit_body(body);
    let mut mentioned = IndexVec::from_elem_n(0, body.local_decls.len());
    Mentioned(&mut mentioned).visit_body(body);
    let mut simple_copies = BitSet::new_empty(body.local_decls.len());
    for ((local, def), count) in unique_def
        .into_iter_enumerated()
        .zip(mentioned.into_iter())
        .skip(body.arg_count + 1)
    {
        if let (Some(true), 2) = (def, count) {
            simple_copies.insert(local);
        }
    }
    // assert!({ simple_copies.intersect(&deref_copies); simple_copies.is_empty()});
    simple_copies.union(&deref_copies);
    simple_copies
}

impl<'tcx> Visitor<'tcx> for DerefCopiesCollector<'_> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, _: Location) {
        if let Rvalue::CopyForDeref(..) = rvalue {
            assert!(place.as_local().is_some());
            let local = place.local;
            self.0.insert(local);
        }
    }
}

struct UniqueDef<'this>(&'this mut IndexVec<Local, Option<bool>>);

impl Visitor<'_> for UniqueDef<'_> {
    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if let PlaceContext::MutatingUse(MutatingUseContext::Store) = context {
            self.visit_place(&Place::from(local), context, location)
        }
    }

    fn visit_place(&mut self, place: &Place<'_>, context: PlaceContext, _: Location) {
        if place.as_local().is_some()
            && matches!(
                context,
                PlaceContext::MutatingUse(MutatingUseContext::Store)
            )
        {
            match self.0[place.local] {
                Some(true) => self.0[place.local] = Some(false),
                Some(false) => {}
                None => self.0[place.local] = Some(true),
            }
        }
    }
}

struct Mentioned<'this>(&'this mut IndexVec<Local, usize>);

impl Visitor<'_> for Mentioned<'_> {
    fn visit_local(&mut self, local: Local, _: PlaceContext, _: Location) {
        self.0[local] += 1;
    }

    fn visit_place(&mut self, place: &Place<'_>, _: PlaceContext, _: Location) {
        self.0[place.local] += 1;
    }

    fn visit_statement(&mut self, statement: &Statement<'_>, location: Location) {
        if let StatementKind::StorageDead(..) | StatementKind::StorageLive(..) = statement.kind {
            return;
        }
        self.super_statement(statement, location);
    }
}
