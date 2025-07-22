//! The Access Path data structure

#![allow(unused)]

mod size_of;
mod ty_post_order;

use std::ops::Range;

use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{HasLocalDecls, Local, Place, ProjectionElem},
    ty::{AdtDef, Ty, TyCtxt},
};
use utils::{rustc::RustProgram, rustc_hash::FxHashMap, smallvec::SmallVec, smallvec::smallvec};

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
        let Path { base, projections } = self;
        Path::new(f(base), projections)
    }
}

/// We erase all other projection kinds except for field
/// selections and dereferences.
pub enum CanonicalProjectionElem {
    Field(usize),
    Deref,
}

impl CanonicalProjectionElem {
    pub fn try_from_projection_elem<V, T>(projection: ProjectionElem<V, T>) -> Option<Self> {
        match projection {
            ProjectionElem::Deref => Some(CanonicalProjectionElem::Deref),
            ProjectionElem::Field(field_idx, _) => {
                Some(CanonicalProjectionElem::Field(field_idx.index()))
            }
            ProjectionElem::Index(_) => None,
            ProjectionElem::ConstantIndex { .. } => None,
            ProjectionElem::Subslice { .. } => None,
            ProjectionElem::Downcast(..) => None,
            ProjectionElem::OpaqueCast(_) => None,
            ProjectionElem::UnwrapUnsafeBinder(_) => None,
            ProjectionElem::Subtype(_) => None,
        }
    }
}

pub type CanonicalProjections = SmallVec<[CanonicalProjectionElem; 2]>;

/// Semantically, an access path represents all postfix pointers
/// reachable from a [`Place`]-like expression. It is conservative
/// in the sense that we do not differentiate array indices (see
/// [`CanonicalProjectionElem`])
pub type CanonicalAccessPath<B> = Path<B, CanonicalProjections>;

pub type EncodedProjections = Range<usize>;

/// An [`EncodedAccessPath`] has the same semantics as a
/// [`CanonicalAccessPath`], but has its concrete projections
/// replaced with `(start_offset, end_offset)`. This pair of
/// offsets reflect the pre-order travesals of all postfix
/// pointers.
pub type EncodedAccessPath<B> = Path<B, EncodedProjections>;

fn encode<'tcx>(
    path: &CanonicalAccessPath<Ty<'tcx>>,
    tcx: TyCtxt<'tcx>,
) -> EncodedAccessPath<Ty<'tcx>> {
    let mut start_offset = 0;
    let mut end_offset = 0; // FIXME: get the size of `path.base`

    todo!()
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

fn canonicalize<'tcx, P: AsRef<Place<'tcx>>>(place: P) -> CanonicalAccessPath<Local> {
    let canonical_projections = place
        .as_ref()
        .iter_projections()
        .filter_map(|(_, projection)| CanonicalProjectionElem::try_from_projection_elem(projection))
        .collect::<SmallVec<_>>();

    Path {
        base: place.as_ref().local,
        projections: canonical_projections,
    }
}
