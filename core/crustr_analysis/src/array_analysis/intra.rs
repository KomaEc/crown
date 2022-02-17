use std::{collections::HashMap, marker::PhantomData};

use crate::{
    array_analysis::{Constraint, ConstraintIdx, CrateLambdaCtxt, Lambda, LambdaData},
    def_use::{BorrowckDefUse, DefUseCategorisable},
    ssa::{
        body_ext::BodyExt,
        rename::{
            handler::{LocalSimplePtrCVMap, SSANameMap},
            impls::PlainRenamer,
            HasSSANameHandler, HasSSARenameState, SSANameHandler, SSARename, SSARenameState,
        },
    },
    FieldDefIdx,
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

impl CrateLambdaCtxt {
    pub fn intra_view(&mut self, body_idx: usize) -> CrateLambdaCtxtIntraView<'_> {
        let lambda_ctxt = &mut self.body_ctxt[body_idx];
        CrateLambdaCtxtIntraView {
            body: body_idx,
            lambda_map: &mut self.lambda_map,
            field_def: &self.field_def,
            local: &mut lambda_ctxt.local,
            local_nested: &lambda_ctxt.local_nested,
        }
    }
}

pub struct Infer<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
{
    pub ctxt: InferCtxt<'infercx, 'tcx>,
    pub ssa_state: SSARenameState<Local>,
    pub extra_handlers: Handler,
    pub _marker: PhantomData<*const DefUse>,
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSARenameState<Local> for Infer<'infercx, 'tcx, DefUse, Handler>
{
    #[inline]
    fn state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.ssa_state
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSANameHandler for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Output = Lambda;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_def(local, idx, location);
        self.ctxt.handle_def(local, idx, location)
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.extra_handlers.handle_use(local, idx, location);
        self.ctxt.handle_use(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.extra_handlers
            .handle_def_at_phi_node(local, idx, block);
        self.ctxt.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.extra_handlers
            .handle_use_at_phi_node(local, idx, block, pos);
        self.ctxt.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    HasSSANameHandler for Infer<'infercx, 'tcx, DefUse, Handler>
{
    type Handler = Self;
    #[inline]
    fn ssa_name_handler(&mut self) -> &mut Self::Handler {
        self
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    SSARename<'tcx> for Infer<'infercx, 'tcx, DefUse, Handler>
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
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    fn process_lhs(&mut self, place: &Place<'tcx>, location: Location) -> Lambda {
        match place.as_ref().last_projection() {
            Some((place, proj)) => {
                todo!()
            }
            None => {
                let ssa_idx = self.define(place.local);
                self.ssa_name_handler()
                    .handle_use(place.local, ssa_idx, location)
            }
        }
    }
}

impl<'infercx, 'tcx, DefUse: DefUseCategorisable, Handler: SSANameHandler<Output = ()>>
    Visitor<'tcx> for Infer<'infercx, 'tcx, DefUse, Handler>
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
        let lhs = self.process_lhs(place, location);
        todo!();
        self.super_assign(place, rvalue, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.super_terminator(terminator, location)
    }
}

pub struct CrateLambdaCtxtIntraView<'intracx> {
    pub body: usize,
    pub lambda_map: &'intracx mut IndexVec<Lambda, LambdaData>,
    pub field_def: &'intracx IndexVec<FieldDefIdx, Vec<Lambda>>,
    pub local: &'intracx mut IndexVec<Local, Vec<Lambda>>,
    pub local_nested: &'intracx IndexVec<Local, Vec<Lambda>>,
}

impl<'intracx> CrateLambdaCtxtIntraView<'intracx> {
    pub fn define(&mut self, base: Local, ssa_idx: usize) -> Lambda {
        let lambda = self.lambda_map.push(LambdaData::Local {
            body: self.body,
            base,
            ssa_idx,
        });
        self.local[base].push(lambda);
        lambda
    }
}

pub struct InferCtxt<'infercx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'infercx Body<'tcx>,
    lambda_ctxt: CrateLambdaCtxtIntraView<'infercx>,
    phi_node_equality_group: IndexVec<BasicBlock, SmallVec<[(Local, Vec<Lambda>); 3]>>,
}

impl<'infercx, 'tcx> SSANameHandler for InferCtxt<'infercx, 'tcx> {
    type Output = Lambda;

    /// TODO: def is also use in this analysis
    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        self.lambda_ctxt.define(local, idx)
    }

    /// TODO: def is also use in this analysis
    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        let lambda = self.lambda_ctxt.define(local, idx);
        self.phi_node_equality_group[block]
            .iter_mut()
            .find(|x| x.0 == local)
            .unwrap()
            .1
            .push(lambda);
    }

    fn handle_use(&mut self, local: Local, idx: usize, _location: Location) -> Self::Output {
        self.lambda_ctxt.local[local][idx]
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, _pos: usize) {
        let lambda = self.lambda_ctxt.local[local][idx];
        self.phi_node_equality_group[block]
            .iter_mut()
            .find(|x| x.0 == local)
            .unwrap()
            .1
            .push(lambda);
    }
}
