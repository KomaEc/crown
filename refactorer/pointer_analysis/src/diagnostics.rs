//! Pointer analyzer diagnostics.

use rustc_errors::DiagnosticBuilder;
use rustc_hir as hir;
use rustc_hir::def::Namespace;
use rustc_hir::def_id::DefId;
use rustc_hir::lang_items::LangItemGroup;
use rustc_hir::GeneratorKind;
use rustc_middle::mir::{
    AggregateKind, Constant, FakeReadCause, Field, Local, LocalInfo, LocalKind, Location, Operand,
    Place, PlaceRef, ProjectionElem, Rvalue, Statement, StatementKind, Terminator, TerminatorKind,
};
use rustc_middle::ty::print::Print;
use rustc_middle::ty::{self, DefIdTree, Instance, Ty, TyCtxt};
use rustc_span::{hygiene::DesugaringKind, symbol::sym, Span};
use rustc_target::abi::VariantIdx;