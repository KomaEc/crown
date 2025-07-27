//! An access path encode the entire set of pointers reachable from
//! an expression through a sequence of projection operations (dereferences
//! and field selections). An access path needs to be k-limited where
//! recursive data structures exist.
//!
//! # Examples
//!
//! ```
//! struct S {
//!     f: *mut i32,
//!     g: *mut f64,
//! }
//! let mut x: *mut *mut S;
//! ```
//! The access path `*x` encode `*x`, `(**x).f` and `(**x).g`.
//!

// #![allow(unused)]

pub mod ctxt;
mod matcher;
mod sizeofable;
mod struct_lookup;
#[cfg(test)]
mod test;

use std::{borrow::Borrow, ops::Range};

use rustc_middle::{
    mir::{HasLocalDecls, Local, Place, ProjectionElem},
    ty::{Ty, TyCtxt},
};
use utils::smallvec::SmallVec;

use crate::access_path::{ctxt::AccessPathsCx, sizeofable::SizeOfable, struct_lookup::StructIndex};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Path<B, P> {
    pub(crate) base: B,
    pub(crate) projections: P,
}

impl<B, P> Path<B, P> {
    pub fn new(base: B, projections: P) -> Self {
        Self { base, projections }
    }

    pub fn map_base<C, F>(self, f: F) -> Path<C, P>
    where
        F: FnOnce(B) -> C,
    {
        let Path {
            base, projections, ..
        } = self;
        Path::new(f(base), projections)
    }
}

/// We erase all other projection kinds except for field
/// selections and dereferences.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CanonicalProjectionElem {
    Select(StructIndex, usize),
    Deref,
}

pub type CanonicalProjections = SmallVec<[CanonicalProjectionElem; 2]>;

/// Semantically, an access path represents all postfix pointers
/// reachable from a [`Place`]-like expression. It is conservative
/// in the sense that we do not differentiate array indices (see
/// [`CanonicalProjectionElem`])
pub type CanonicalAccessPath<B> = Path<B, CanonicalProjections>;

/// We define our own type so as to derive [`Copy`].
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct EncodedProjections {
    start_offset: usize,
    end_offset: usize,
}

impl EncodedProjections {
    #[inline]
    pub fn size(&self) -> usize {
        self.end_offset - self.start_offset
    }
}

impl From<Range<usize>> for EncodedProjections {
    fn from(value: Range<usize>) -> Self {
        EncodedProjections {
            start_offset: value.start,
            end_offset: value.end,
        }
    }
}

impl From<EncodedProjections> for Range<usize> {
    fn from(value: EncodedProjections) -> Self {
        value.start_offset..value.end_offset
    }
}

/// An [`EncodedAccessPath`] has the same semantics as a
/// [`CanonicalAccessPath`], but has its concrete projections
/// replaced with `(start_offset, end_offset)`. This pair of
/// offsets (represented by [`EncodedProjections`]) reflect
/// the pre-order travesals of all postfix pointers.
///
/// [`EncodedProjections`] is k-limited where recursive data
/// structures exist.
pub type EncodedAccessPath<B> = Path<B, KLimited<EncodedProjections>>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct KLimited<T> {
    pub(crate) k_limit: usize,
    pub(crate) data: T,
}

impl<T> KLimited<T> {
    pub fn map<U, F>(self, f: F) -> KLimited<U>
    where
        F: FnOnce(T) -> U,
    {
        KLimited {
            k_limit: self.k_limit,
            data: f(self.data),
        }
    }
}

impl<T> KLimited<T> {
    pub fn new(k_limit: usize, data: T) -> Self {
        KLimited { k_limit, data }
    }
}

/// Decompose a type into levels of outside pointers and a (possible) adt
fn peel_pointers(ty: Ty) -> (usize, Ty) {
    let mut ty = ty;

    let mut num_pointers = 0;
    loop {
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }

        if let Some(ty_mut) = ty.builtin_deref(true) {
            ty = ty_mut;
            num_pointers += 1;
            continue;
        }

        break;
    }

    (num_pointers, ty)
}

impl<SizeOf: SizeOfable> AccessPathsCx<SizeOf> {
    pub fn canonicalize<'tcx, P, D>(
        &self,
        path: P,
        local_decls: &D,
        tcx: TyCtxt<'tcx>,
    ) -> CanonicalAccessPath<Local>
    where
        P: Borrow<Place<'tcx>>,
        D: HasLocalDecls<'tcx>,
    {
        let place = path.borrow();
        let canonical_projections = place
            .iter_projections()
            .filter_map(|(place, projection)| match projection {
                ProjectionElem::Deref => Some(CanonicalProjectionElem::Deref),
                ProjectionElem::Field(field_idx, _) => {
                    let base_ty = place.ty(local_decls, tcx).ty;

                    // FIXME: what about tuple structs?

                    base_ty
                        .ty_adt_def()
                        .and_then(|adt_def| self.struct_lookup.try_index(adt_def.did()))
                        .map(|struct_index| {
                            CanonicalProjectionElem::Select(struct_index, field_idx.index())
                        })
                }
                ProjectionElem::Index(_) => None,
                ProjectionElem::ConstantIndex { .. } => None,
                ProjectionElem::Subslice { .. } => None,
                ProjectionElem::Downcast(..) => None,
                ProjectionElem::OpaqueCast(_) => None,
                ProjectionElem::UnwrapUnsafeBinder(_) => None,
                ProjectionElem::Subtype(_) => None,
            })
            .collect::<SmallVec<_>>();

        Path::new(place.local, canonical_projections)
    }

    pub fn encode<'tcx, P, D>(
        &self,
        path: KLimited<P>,
        local_decls: &D,
        tcx: TyCtxt<'tcx>,
    ) -> EncodedAccessPath<Local>
    where
        P: Borrow<Place<'tcx>>,
        D: HasLocalDecls<'tcx>,
    {
        let place = path.data.borrow();

        let canonical_access_path = self.canonicalize(place, local_decls, tcx);

        let mut start_offset = 0;
        let mut num_indirections = 0;

        for &projection_elem in &canonical_access_path.projections {
            if num_indirections == path.k_limit {
                return EncodedAccessPath {
                    base: place.local,
                    projections: KLimited::new(0, (start_offset..start_offset).into()),
                };
            }

            match projection_elem {
                CanonicalProjectionElem::Select(struct_index, field_idx) => {
                    start_offset += self.size_of.start_offset(
                        KLimited {
                            k_limit: path.k_limit - num_indirections,
                            data: struct_index,
                        },
                        field_idx,
                    )
                }
                CanonicalProjectionElem::Deref => {
                    start_offset += 1;
                    num_indirections += 1;
                }
            }
        }

        let end_offset = start_offset
            + self.size_of(KLimited::new(
                path.k_limit - num_indirections,
                place.ty(local_decls, tcx).ty,
            ));

        EncodedAccessPath {
            base: place.local,
            projections: KLimited::new(
                path.k_limit - num_indirections,
                (start_offset..end_offset).into(),
            ),
        }
    }
}
