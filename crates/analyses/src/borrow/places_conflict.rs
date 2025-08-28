//! Simplications of https://doc.rust-lang.org/beta/nightly-rustc/src/rustc_borrowck/places_conflict.rs.html

use rustc_middle::{
    mir::{Body, Place, PlaceElem, PlaceRef, ProjectionElem},
    ty::TyCtxt,
};
use std::cmp::max;

/// When checking if a place conflicts with another place, this enum is used to influence decisions
/// where a place might be equal or disjoint with another place, such as if `a[i] == a[j]`.
/// `PlaceConflictBias::Overlap` would bias toward assuming that `i` might equal `j` and that these
/// places overlap. `PlaceConflictBias::NoOverlap` assumes that for the purposes of the predicate
/// being run in the calling context, the conservative choice is to assume the compared indices
/// are disjoint (and therefore, do not overlap).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlaceConflictBias {
    Overlap,
    NoOverlap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The degree of overlap between 2 places for borrow-checking.
pub(crate) enum Overlap {
    /// The places might partially overlap - in this case, we give
    /// up and say that they might conflict. This occurs when
    /// different fields of a union are borrowed. For example,
    /// if `u` is a union, we have no way of telling how disjoint
    /// `u.a.x` and `a.b.y` are.
    Arbitrary,
    /// The places have the same type, and are either completely disjoint
    /// or equal - i.e., they can't "partially" overlap as can occur with
    /// unions. This is the "base case" on which we recur for extensions
    /// of the place.
    EqualOrDisjoint,
    /// The places are disjoint, so we know all extensions of them
    /// will also be disjoint.
    Disjoint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccessDepth {
    Shallow,
    Deep,
}

pub fn places_conflict<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    borrow_place: Place<'tcx>,
    access_place: Place<'tcx>,
    access_detph: AccessDepth,
    bias: PlaceConflictBias,
) -> bool {
    let borrow_local = borrow_place.local;
    let access_local = access_place.local;

    if borrow_local != access_local {
        // We have proven the borrow disjoint - further projections will remain disjoint.
        return false;
    }

    // This Local/Local case is handled by the more general code below, but
    // it's so common that it's a speed win to check for it first.
    if borrow_place.projection.is_empty() && access_place.projection.is_empty() {
        return true;
    }

    place_components_conflict(
        tcx,
        body,
        borrow_place,
        access_place.as_ref(),
        access_detph,
        bias,
    )
}

fn place_components_conflict<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    borrow_place: Place<'tcx>,
    access_place: PlaceRef<'tcx>,
    access_detph: AccessDepth,
    bias: PlaceConflictBias,
) -> bool {
    let borrow_local = borrow_place.local;
    let access_local = access_place.local;
    // borrow_conflicts_with_place should have checked that.
    assert_eq!(borrow_local, access_local);

    // loop invariant: borrow_c is always either equal to access_c or disjoint from it.
    for ((borrow_place, borrow_c), &access_c) in
        borrow_place.iter_projections().zip(access_place.projection)
    {
        // Borrow and access path both have more components.
        //
        // Examples:
        //
        // - borrow of `a.(...)`, access to `a.(...)`
        // - borrow of `a.(...)`, access to `b.(...)`
        //
        // Here we only see the components we have checked so
        // far (in our examples, just the first component). We
        // check whether the components being borrowed vs
        // accessed are disjoint (as in the second example,
        // but not the first).
        match place_projection_conflict(tcx, body, borrow_place, borrow_c, access_c, bias) {
            Overlap::Arbitrary => {
                // We have encountered different fields of potentially
                // the same union - the borrow now partially overlaps.
                //
                // There is no *easy* way of comparing the fields
                // further on, because they might have different types
                // (e.g., borrows of `u.a.0` and `u.b.y` where `.0` and
                // `.y` come from different structs).
                //
                // We could try to do some things here - e.g., count
                // dereferences - but that's probably not a good
                // idea, at least for now, so just give up and
                // report a conflict. This is unsafe code anyway so
                // the user could always use raw pointers.
                return true;
            }
            Overlap::EqualOrDisjoint => {
                // This is the recursive case - proceed to the next element.
            }
            Overlap::Disjoint => {
                // We have proven the borrow disjoint - further
                // projections will remain disjoint.
                return false;
            }
        }
    }

    if borrow_place.projection.len() > access_place.projection.len() {
        for (_, elem) in borrow_place
            .iter_projections()
            .skip(access_place.projection.len())
        {
            // Borrow path is longer than the access path. Examples:
            //
            // - borrow of `a.b.c`, access to `a.b`
            //
            // Here, we know that the borrow can access a part of
            // our place. This is a conflict if that is a part our
            // access cares about.

            if matches!(
                (elem, access_detph),
                (ProjectionElem::Deref, AccessDepth::Shallow)
            ) {
                return false;
            }
        }
    }

    true
}

// Given that the bases of `elem1` and `elem2` are always either equal
// or disjoint (and have the same type!), return the overlap situation
// between `elem1` and `elem2`.
fn place_projection_conflict<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    pi1: PlaceRef<'tcx>,
    pi1_elem: PlaceElem<'tcx>,
    pi2_elem: PlaceElem<'tcx>,
    bias: PlaceConflictBias,
) -> Overlap {
    match (pi1_elem, pi2_elem) {
        (ProjectionElem::Deref, ProjectionElem::Deref) => {
            // derefs (e.g., `*x` vs. `*x`) - recur.
            Overlap::EqualOrDisjoint
        }
        (ProjectionElem::OpaqueCast(_), ProjectionElem::OpaqueCast(_)) => {
            // casts to other types may always conflict irrespective of the type being cast to.
            Overlap::EqualOrDisjoint
        }
        (ProjectionElem::Field(f1, _), ProjectionElem::Field(f2, _)) => {
            if f1 == f2 {
                // same field (e.g., `a.y` vs. `a.y`) - recur.
                Overlap::EqualOrDisjoint
            } else {
                let ty = pi1.ty(body, tcx).ty;
                if ty.is_union() {
                    // Different fields of a union, we are basically stuck.
                    Overlap::Arbitrary
                } else {
                    // Different fields of a struct (`a.x` vs. `a.y`). Disjoint!
                    Overlap::Disjoint
                }
            }
        }
        (ProjectionElem::Downcast(_, v1), ProjectionElem::Downcast(_, v2)) => {
            // different variants are treated as having disjoint fields,
            // even if they occupy the same "space", because it's
            // impossible for 2 variants of the same enum to exist
            // (and therefore, to be borrowed) at the same time.
            //
            // Note that this is different from unions - we *do* allow
            // this code to compile:
            //
            // ```
            // fn foo(x: &mut Result<i32, i32>) {
            //     let mut v = None;
            //     if let Ok(ref mut a) = *x {
            //         v = Some(a);
            //     }
            //     // here, you would *think* that the
            //     // *entirety* of `x` would be borrowed,
            //     // but in fact only the `Ok` variant is,
            //     // so the `Err` variant is *entirely free*:
            //     if let Err(ref mut a) = *x {
            //         v = Some(a);
            //     }
            //     drop(v);
            // }
            // ```
            if v1 == v2 {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::Index(..),
            ProjectionElem::Index(..)
            | ProjectionElem::ConstantIndex { .. }
            | ProjectionElem::Subslice { .. },
        )
        | (
            ProjectionElem::ConstantIndex { .. } | ProjectionElem::Subslice { .. },
            ProjectionElem::Index(..),
        ) => {
            // Array indexes (`a[0]` vs. `a[i]`). These can either be disjoint
            // (if the indexes differ) or equal (if they are the same).
            match bias {
                PlaceConflictBias::Overlap => {
                    // If we are biased towards overlapping, then this is the recursive
                    // case that gives "equal *or* disjoint" its meaning.
                    Overlap::EqualOrDisjoint
                }
                PlaceConflictBias::NoOverlap => {
                    // If we are biased towards no overlapping, then this is disjoint.
                    Overlap::Disjoint
                }
            }
        }
        (
            ProjectionElem::ConstantIndex {
                offset: o1,
                min_length: _,
                from_end: false,
            },
            ProjectionElem::ConstantIndex {
                offset: o2,
                min_length: _,
                from_end: false,
            },
        )
        | (
            ProjectionElem::ConstantIndex {
                offset: o1,
                min_length: _,
                from_end: true,
            },
            ProjectionElem::ConstantIndex {
                offset: o2,
                min_length: _,
                from_end: true,
            },
        ) => {
            if o1 == o2 {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::ConstantIndex {
                offset: offset_from_begin,
                min_length: min_length1,
                from_end: false,
            },
            ProjectionElem::ConstantIndex {
                offset: offset_from_end,
                min_length: min_length2,
                from_end: true,
            },
        )
        | (
            ProjectionElem::ConstantIndex {
                offset: offset_from_end,
                min_length: min_length1,
                from_end: true,
            },
            ProjectionElem::ConstantIndex {
                offset: offset_from_begin,
                min_length: min_length2,
                from_end: false,
            },
        ) => {
            // both patterns matched so it must be at least the greater of the two
            let min_length = max(min_length1, min_length2);
            // `offset_from_end` can be in range `[1..min_length]`, 1 indicates the last
            // element (like -1 in Python) and `min_length` the first.
            // Therefore, `min_length - offset_from_end` gives the minimal possible
            // offset from the beginning
            if offset_from_begin >= min_length - offset_from_end {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: false,
            },
            ProjectionElem::Subslice {
                from,
                to,
                from_end: false,
            },
        )
        | (
            ProjectionElem::Subslice {
                from,
                to,
                from_end: false,
            },
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: false,
            },
        ) => {
            if (from..to).contains(&offset) {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: false,
            },
            ProjectionElem::Subslice { from, .. },
        )
        | (
            ProjectionElem::Subslice { from, .. },
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: false,
            },
        ) => {
            if offset >= from {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: true,
            },
            ProjectionElem::Subslice {
                to, from_end: true, ..
            },
        )
        | (
            ProjectionElem::Subslice {
                to, from_end: true, ..
            },
            ProjectionElem::ConstantIndex {
                offset,
                min_length: _,
                from_end: true,
            },
        ) => {
            if offset > to {
                Overlap::EqualOrDisjoint
            } else {
                Overlap::Disjoint
            }
        }
        (
            ProjectionElem::Subslice {
                from: f1,
                to: t1,
                from_end: false,
            },
            ProjectionElem::Subslice {
                from: f2,
                to: t2,
                from_end: false,
            },
        ) => {
            if f2 >= t1 || f1 >= t2 {
                Overlap::Disjoint
            } else {
                Overlap::EqualOrDisjoint
            }
        }
        (ProjectionElem::Subslice { .. }, ProjectionElem::Subslice { .. }) => {
            Overlap::EqualOrDisjoint
        }
        (
            ProjectionElem::Deref
            | ProjectionElem::Field(..)
            | ProjectionElem::Index(..)
            | ProjectionElem::ConstantIndex { .. }
            | ProjectionElem::Subtype(_)
            | ProjectionElem::OpaqueCast { .. }
            | ProjectionElem::Subslice { .. }
            | ProjectionElem::Downcast(..),
            _,
        ) => unreachable!(
            "mismatched projections in place_element_conflict: {:?} and {:?}",
            pi1_elem, pi2_elem
        ),

        (ProjectionElem::UnwrapUnsafeBinder(_), _) => {
            todo!()
        }
    }
}
