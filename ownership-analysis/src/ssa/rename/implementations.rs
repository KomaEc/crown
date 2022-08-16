use super::{
    HasSSANameHandler, HasSSARenameState, HasTyCtxt, SSANameHandler, SSARename, SSARenameState,
};
use crate::{def_use::IsDefUse, ssa::body_ext::BodyExt};
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        Body, Local, Location, Place, Statement, Terminator,
    },
    ty::TyCtxt,
};
use std::marker::PhantomData;

pub struct PlainRenamer<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> {
    tcx: TyCtxt<'tcx>,
    body: &'me Body<'tcx>,
    state: SSARenameState<Local>,
    pub ssa_name_handler: H,
    _marker: PhantomData<*const DefUse>,
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> PlainRenamer<'me, 'tcx, DefUse, H> {
    #[inline]
    pub fn rename(&mut self) {
        self.visit_body(self.body)
    }
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> HasSSARenameState<Local>
    for PlainRenamer<'me, 'tcx, DefUse, H>
{
    #[inline(always)]
    fn ssa_state(&mut self) -> &mut SSARenameState<Local> {
        &mut self.state
    }
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> HasSSANameHandler
    for PlainRenamer<'me, 'tcx, DefUse, H>
{
    type Handler = H;

    type DefUse = DefUse;

    #[inline]
    fn ssa_name_handler(&mut self) -> &mut H {
        &mut self.ssa_name_handler
    }
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> HasTyCtxt<'tcx>
    for PlainRenamer<'me, 'tcx, DefUse, H>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> SSARename<'tcx>
    for PlainRenamer<'me, 'tcx, DefUse, H>
{
    #[inline]
    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.visit_statement(statement, location)
    }

    #[inline]
    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.visit_terminator(terminator, location)
    }
}

impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> PlainRenamer<'me, 'tcx, DefUse, H> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'me Body<'tcx>, rename: H) -> Self {
        PlainRenamer {
            tcx,
            body,
            state: SSARenameState::new(&body.local_decls),
            ssa_name_handler: rename,
            _marker: PhantomData,
        }
    }
}

/// FIXME: should it visit rvalue before place? For statements like `x = x.f`, if
/// place is visited first, then the use on RHS will get a wrong rename!
/// However this is rare, if not impossible, in MIR. I tried several artificial
/// examples, such statements are decomposed by inserting temp varaibles.
impl<'me, 'tcx, DefUse: IsDefUse, H: SSANameHandler> Visitor<'tcx>
    for PlainRenamer<'me, 'tcx, DefUse, H>
{
    fn visit_body(&mut self, body: &Body<'tcx>) {
        self.rename_body(body, &body.compute_phi_node::<DefUse>(self.tcx))
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorise_for_place(place, context, self.body, self.tcx) {
            if IsDefUse::defining(def_use) {
                let i = self.ssa_state().define(place.local);
                self.ssa_name_handler.handle_def(place.local, i, location);
            } else if IsDefUse::using(def_use) {
                let i = self.ssa_state().r#use(place.local);
                self.ssa_name_handler.handle_use(place.local, i, location);
            }
        }
    }

    fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorise_for_local(local, &self.body.local_decls, context)
        {
            if IsDefUse::defining(def_use) {
                let i = self.ssa_state().define(local);
                self.ssa_name_handler.handle_def(local, i, location);
            } else if IsDefUse::using(def_use) {
                let i = self.ssa_state().r#use(local);
                self.ssa_name_handler.handle_use(local, i, location);
            }
        }
    }
}
