use crate::{
    array_analysis::{Constraint, Lambda},
    def_use::BorrowckDefUse,
    ssa::{
        body_ext::BodyExt,
        rename::{
            handler::{LocalSimplePtrCVMap, SSANameMap},
            impls::PlainRenamer,
        },
    },
};
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{visit::Visitor, Body, Location, Place, Rvalue, Terminator, TerminatorKind},
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
pub struct IntraContext<'intracx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'intracx Body<'tcx>,
    pub assumptions: IndexVec<Lambda, Option<bool>>,
    pub constraint_set: IndexVec<ConstraintId, Constraint>,
    pub ssa_name_map: SSANameMap,
    pub cv: LocalSimplePtrCVMap<'intracx, 'tcx, Lambda>,
}

impl<'intracx, 'tcx> IntraContext<'intracx, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'intracx Body<'tcx>) -> Self {
        let insertion_points = body.compute_phi_node::<BorrowckDefUse>(tcx);
        let mut renamer = PlainRenamer::<BorrowckDefUse, (SSANameMap, LocalSimplePtrCVMap<Lambda>)>::new(
            tcx,
            body,
            (
                SSANameMap::new(body, &insertion_points),
                LocalSimplePtrCVMap::new(body),
            ),
        );
        renamer.rename();
        IntraContext {
            tcx,
            body,
            assumptions: IndexVec::from_elem(None, &renamer.ssa_name_handler.1.rev_map),
            constraint_set: IndexVec::new(),
            ssa_name_map: renamer.ssa_name_handler.0,
            cv: renamer.ssa_name_handler.1,
        }
    }
}

impl<'intracx, 'tcx> Visitor<'tcx> for IntraContext<'intracx, 'tcx> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if place.ty(self.body, self.tcx).ty.is_any_ptr() {
            let local = place.as_local().unwrap();
        }
    }
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                cleanup: _,
                from_hir_call,
                fn_span: _,
            } => {
                if let FnDef(def_id, generic_args) = func.constant().unwrap().ty().kind() {
                    // assert!(generic_args.is_empty());
                    println!("calling {}", self.tcx.def_path_str(*def_id))
                }
            }
            _ => {}
        }
    }
}
