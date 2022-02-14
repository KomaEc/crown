use crate::{
    array_analysis::{Constraint, Lambda},
    def_use::BorrowckDefUse,
    ssa::{
        body_ext::BodyExt,
        rename::{
            handler::{LocalSimplePtrCVMap, SSANameMap},
            impls::PlainRenamer, SSARenameState, HasSSARenameState, HasSSANameHandler, SSARename,
        },
    },
};
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Body, Local, Location, Place, Rvalue, Terminator, TerminatorKind, Statement},
    ty::{TyCtxt, TyKind::FnDef},
};

pub struct BodySummary<'body, 'tcx> {
    pub body: &'body Body<'tcx>,
    pub assumptions: IndexVec<Lambda, Option<bool>>,
    pub constraint_set: Vec<Constraint>,
}

rustc_index::newtype_index! {
    pub struct ConstraintId {
        DEBUG_FORMAT = "constraint {}"
    }
}

/// Assume no crate struct definintion and local nested pointer type
pub struct IntraCtxt<'intracx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'intracx Body<'tcx>,
    pub ssa_state: SSARenameState<Local>,
    pub assumptions: IndexVec<Lambda, Option<bool>>,
    pub constraint_set: IndexVec<ConstraintId, Constraint>,
    pub extra_handlers: (SSANameMap, LocalSimplePtrCVMap<'intracx, 'tcx, Lambda>),
}

impl<'intracx, 'tcx> HasSSARenameState<Local> for IntraCtxt<'intracx, 'tcx> {
    #[inline]
    fn state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'intracx, 'tcx> HasSSANameHandler for IntraCtxt<'intracx, 'tcx> {
    type Handler = (SSANameMap, LocalSimplePtrCVMap<'intracx, 'tcx, Lambda>);
    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        &mut self.extra_handlers
    }
}

impl<'intracx, 'tcx> SSARename<'tcx> for IntraCtxt<'intracx, 'tcx> {
    type DefUse = BorrowckDefUse;

    #[inline]
    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.visit_statement(statement, location)
    }

    #[inline]
    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.visit_terminator(terminator, location)
    }
}

impl<'intracx, 'tcx> Visitor<'tcx> for IntraCtxt<'intracx, 'tcx> {}