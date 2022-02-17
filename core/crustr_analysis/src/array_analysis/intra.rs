use std::{collections::HashMap, marker::PhantomData};

use crate::{
    array_analysis::{Constraint, ConstraintIdx, Lambda},
    def_use::{BorrowckDefUse, DefUseCategorisable},
    ssa::{
        body_ext::BodyExt,
        rename::{
            handler::{LocalSimplePtrCVMap, SSANameMap},
            impls::PlainRenamer,
            HasSSANameHandler, HasSSARenameState, SSANameHandler, SSARename, SSARenameState,
        },
    },
};
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Body, Local, Location, Operand, Place, Rvalue, Statement, Terminator,
        TerminatorKind,
    },
    ty::{TyCtxt, TyKind::FnDef},
};
use smallvec::SmallVec;

pub struct InferCtxt<
    'infercx,
    'tcx,
    DefUse: DefUseCategorisable,
    Handler: SSANameHandler<Output = ()>,
> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'infercx Body<'tcx>,
    pub ssa_state: SSARenameState<Local>,
    pub assumptions: IndexVec<Lambda, Option<bool>>,
    pub constraint_set: IndexVec<ConstraintIdx, Constraint>,
    pub phi_node_equality_group: IndexVec<BasicBlock, SmallVec<[(Local, Vec<Lambda>); 3]>>,
    pub extra_handlers: Handler, //(SSANameMap, LocalSimplePtrCVMap<'infercx, 'tcx, Lambda>),
    pub _marker: PhantomData<*const DefUse>,
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSARenameState<Local> for InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSANameHandler for InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    type Output = Lambda;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        todo!()
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.phi_node_equality_group[block]
            .iter_mut()
            .find(|x| x.0 == local)
            .unwrap()
            .1
            .push(todo!());
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        todo!()
    }

    fn handle_use_at_phi_node(
        &mut self,
        local: Local,
        idx: usize,
        block: rustc_middle::mir::BasicBlock,
        pos: usize,
    ) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        todo!()
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSANameHandler for InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    type Handler = Self;
    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSARename<'tcx> for InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    type DefUse = DefUse;

    #[inline]
    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.visit_statement(statement, location)
    }

    #[inline]
    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.visit_terminator(terminator, location)
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    /// TODO: assume no nested types
    fn process_lhs(&mut self, local: Local, location: Location) -> (Local, usize) {
        let def = self.define(local);
        self.ssa_name_handler().handle_def(local, def, location);
        (local, def)
    }

    fn process_rhs(&mut self, local: Local, location: Location) -> (Local, usize) {
        let r#use = self.r#use(local);
        self.ssa_name_handler().handle_use(local, r#use, location);
        (local, r#use)
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    Visitor<'tcx> for InferCtxt<'infercx, 'tcx, DefUse, Handler>
{
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorize(context) {
            if DefUse::defining(def_use) {
                let i = self.define(local);
                self.ssa_name_handler().handle_def(local, i, location);
            } else if DefUse::using(def_use) {
                let i = self.r#use(local);
                self.ssa_name_handler().handle_use(local, i, location);
            }
        }
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        let ty = place.ty(self.body, self.tcx).ty;
        if ty.is_any_ptr() && !ty.is_fn_ptr() {
            if let Some(lhs) = place.as_local() {
                let (lhs, def) = self.process_lhs(lhs, location);
                match rvalue {
                    Rvalue::Use(Operand::Move(place)) | Rvalue::Use(Operand::Copy(place)) => {
                        if let Some(rhs) = place.as_local() {
                            let (rhs, r#use) = self.process_rhs(rhs, location);

                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.super_assign(place, rvalue, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.super_terminator(terminator, location)
    }
}
