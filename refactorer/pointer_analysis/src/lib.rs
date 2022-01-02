#![feature(rustc_private)]
#![feature(min_specialization)]
#![feature(box_patterns)]
#![feature(crate_visibility_modifier)]
#![feature(maybe_uninit_extra)]

#[macro_use]
extern crate index;

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_target;

use crate::ctxt::PointerAnalysisCtxt;
use index::vec::IndexVec;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Body, PlaceRef},
    ty::TyCtxt,
};
use std::ops::Index;

pub mod andersen;
mod constraint_generation;
mod ctxt;
pub mod steensgaard;

pub struct PointerAnalysis;

impl<'aa, 'tcx> PointerAnalysis {
    pub fn new_analysis(
        bodies: &'aa mut [&'aa Body<'tcx>],
        tcx: TyCtxt<'tcx>,
    ) -> PointerAnalysisCtxt<'aa, 'tcx> {
        bodies.sort_by_key(|body| body.source.instance.def_id());
        PointerAnalysisCtxt::new(bodies, tcx)
    }
}

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
}

impl ConstraintSet {
    pub fn new() -> ConstraintSet {
        ConstraintSet {
            constraints: IndexVec::new(),
        }
    }

    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    #[inline]
    pub fn push(&mut self, c: Constraint) -> ConstraintIndex {
        self.constraints.push(c)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (ConstraintIndex, &Constraint)> {
        self.constraints.iter_enumerated()
    }

    #[inline]
    pub fn universe(&self) -> &IndexVec<ConstraintIndex, Constraint> {
        &self.constraints
    }
}

impl Index<ConstraintIndex> for ConstraintSet {
    type Output = Constraint;

    fn index(&self, i: ConstraintIndex) -> &Self::Output {
        &self.constraints[i]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ConstraintKind {
    /// p = &q
    AddressOf,
    /// p = q
    Copy,
    /// p = *q
    Load,
    /// *p = q
    Store,
}

#[derive(Clone, Copy)]
pub struct Constraint {
    pub constraint_kind: ConstraintKind,
    pub left: PointerAnalysisNode,
    pub right: PointerAnalysisNode,
}

impl Constraint {
    pub fn new(ck: ConstraintKind, l: PointerAnalysisNode, r: PointerAnalysisNode) -> Self {
        Constraint {
            constraint_kind: ck,
            left: l,
            right: r,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PointerAnalysisNodeData<'tcx> {
    Mir(LocalDefId, PlaceRef<'tcx>),
    Temporary(LocalDefId),
}

impl<'tcx> From<(LocalDefId, PlaceRef<'tcx>)> for PointerAnalysisNodeData<'tcx> {
    fn from(data: (LocalDefId, PlaceRef<'tcx>)) -> Self {
        PointerAnalysisNodeData::Mir(data.0, data.1)
    }
}

impl<'tcx> From<LocalDefId> for PointerAnalysisNodeData<'tcx> {
    fn from(did: LocalDefId) -> Self {
        PointerAnalysisNodeData::Temporary(did)
    }
}

index::newtype_index! {
    pub struct PointerAnalysisNode {
        DEBUG_FORMAT = "AndersenNode({})"
    }
}

impl PointerAnalysisNode {
    pub fn kopy(self, other: PointerAnalysisNode) -> Constraint {
        Constraint::new(ConstraintKind::Copy, self, other)
    }

    pub fn load(self, other: PointerAnalysisNode) -> Constraint {
        Constraint::new(ConstraintKind::Load, self, other)
    }

    pub fn store(self, other: PointerAnalysisNode) -> Constraint {
        Constraint::new(ConstraintKind::Store, self, other)
    }

    pub fn get_address_of(self, other: PointerAnalysisNode) -> Constraint {
        Constraint::new(ConstraintKind::AddressOf, self, other)
    }
}

index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
