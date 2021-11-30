use rustc_middle::mir::visit::{
    MutatingUseContext, NonMutatingUseContext, PlaceContext, TyContext, Visitor,
};
use rustc_middle::mir::ProjectionElem;
use rustc_middle::mir::{
    Body, BorrowKind, Local, LocalDecl, Location, Mutability, NullOp, Place, PlaceRef, Rvalue,
};
use rustc_middle::ty::TyCtxt;

use crate::{
    andersen::node_generation::NodeGeneration,
    andersen::{AndersenNode, ConstraintSet},
};

// pub type PtsGraph = Graph<AndersenNode, ()>;

/// Data structure for constraint generation.
/// 'cg = the duration of the constraint generation
pub struct ConstraintGeneration<'cg, 'tcx> {
    constraints: ConstraintSet,
    node_generation: NodeGeneration<'tcx>,
    body: &'cg Body<'tcx>,
    tcx: TyCtxt<'tcx>,
}

impl<'cg, 'tcx> Visitor<'tcx> for ConstraintGeneration<'cg, 'tcx> {
    /// Default visitor will visit basic blocks before local declarations,
    /// so we overwrite here.
    fn visit_body(&mut self, body: &Body<'tcx>) {
        log::trace!("visiting body");
        for (local, decl) in body.local_decls.iter_enumerated() {
            self.visit_local_decl(local, decl)
        }

        for (bb, data) in body.basic_blocks().iter_enumerated() {
            self.visit_basic_block_data(bb, data)
        }
    }

    fn visit_local_decl(&mut self, local: Local, local_decl: &LocalDecl<'tcx>) {
        log::trace!("visiting local declaration {:?} : {}", local, local_decl.ty);
        let LocalDecl {
            mutability: _,
            ty,
            user_ty: _,
            source_info: _,
            internal: _,
            local_info: _,
            is_block_tail: _,
        } = local_decl;

        if ty.is_any_ptr() {
            if ty.is_fn_ptr() {
                log::error!("Function pointer: {} is not supported!", ty);
                unimplemented!()
            }
            // generate andersen node for this local
            let place_ref = Place::from(local).as_ref();
            let _ = self
                .node_generation
                .generate_from_place_ref(place_ref.into());
        }

        self.super_local_decl(local, local_decl)
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        log::trace!(
            "visiting assignment statment {:?} = {:?} at location: {:?}",
            place,
            rvalue,
            location
        );

        let place_ty = place.ty(self.body, self.tcx);
        if place_ty.ty.is_any_ptr() {
            if place_ty.ty.is_fn_ptr() {
                log::error!("Function pointer: {} is not supported!", place_ty.ty);
                unimplemented!()
            }
            self.process_rvalue_of_ptr_ty(rvalue, location);
            self.process_place_of_ptr_ty(
                place,
                PlaceContext::MutatingUse(MutatingUseContext::Store),
                location,
            );
        }

        self.super_assign(place, rvalue, location)
    }
}

impl<'cg, 'tcx> ConstraintGeneration<'cg, 'tcx> {
    pub fn new(body: &'cg Body<'tcx>, tcx: TyCtxt<'tcx>) -> ConstraintGeneration<'cg, 'tcx> {
        ConstraintGeneration {
            constraints: ConstraintSet::new(),
            node_generation: NodeGeneration::new(),
            body,
            tcx,
        }
    }

    /// Process place of pointer type, return an Andersen node representing this place. Return true
    /// if this place is indirect
    ///
    /// If `place` is nested, for instance, `*(*(*p).0).1`, introduce temporary variables implicitly.
    /// In the above example, temp vars are introduced as:
    /// ```mir
    /// tmp1 = *p;
    /// tmp2 = *tmp2;
    /// tmp3 = *tmp2;
    /// tmp3 = ... // or ... = tmp3
    /// ```
    /// , and `andersen_repr(tmp3)` is returned
    /// Note that, the current analysis is flow insensitive, meaning that assignment to `x.f` is treated
    /// the same as assignment to `x`.
    fn process_place_of_ptr_ty(
        &mut self,
        place: &Place<'tcx>,
        _context: PlaceContext,
        location: Location,
    ) -> (AndersenNode, bool) {
        log::trace!("processing place {:?} at location {:?}", place, location);

        //for (place_ref, _) in place.iter_projections() {
        //    let _ = self.node_generation.generate(place_ref.into());
        //}

        let mut repr = self.node_generation.generate_from_place_ref(place.as_ref());

        /// FIXME: wrong logic. should return `p` if `*p` is the place, and generate a [`Load`] or [`Store`]
        /// constraint. Otherwise [`Copy`] or [`AddressOf`] constraints. 
        if let Some(_local) = place.local_or_deref_local() {
        } else {
            /// FIXME: introduce temporary variable!!!!
            for (place_ref, proj_elem) in place.iter_projections() {
                log::trace!(
                    "--> processing place {:?} with elem {:?}",
                    place_ref,
                    proj_elem
                );
                match proj_elem {
                    ProjectionElem::Deref => {
                        unimplemented!()
                    }
                    _ => continue,
                }
            }
        }
        (repr, unimplemented!())
    }

    fn process_rvalue_of_ptr_ty(&mut self, rvalue: &Rvalue<'tcx>, location: Location)
    /* -> AndersenNode */
    {
        match rvalue {
            Rvalue::Use(operand) => {
                self.visit_operand(operand, location);
            }

            Rvalue::Ref(r, bk, path) => {
                self.visit_region(r, location);
                let ctx = match bk {
                    BorrowKind::Shared => {
                        PlaceContext::NonMutatingUse(NonMutatingUseContext::SharedBorrow)
                    }
                    BorrowKind::Shallow => {
                        PlaceContext::NonMutatingUse(NonMutatingUseContext::ShallowBorrow)
                    }
                    BorrowKind::Unique => {
                        PlaceContext::NonMutatingUse(NonMutatingUseContext::UniqueBorrow)
                    }
                    BorrowKind::Mut { .. } => PlaceContext::MutatingUse(MutatingUseContext::Borrow),
                };
                self.visit_place(path, ctx, location);
            }

            Rvalue::AddressOf(m, path) => {
                let ctx = match m {
                    Mutability::Mut => PlaceContext::MutatingUse(MutatingUseContext::AddressOf),
                    Mutability::Not => {
                        PlaceContext::NonMutatingUse(NonMutatingUseContext::AddressOf)
                    }
                };
                self.visit_place(path, ctx, location);
            }

            Rvalue::Cast(_cast_kind, operand, ty) => {
                self.visit_operand(operand, location);
                self.visit_ty(ty, TyContext::Location(location));
            }

            Rvalue::BinaryOp(_bin_op, box (lhs, rhs))
            | Rvalue::CheckedBinaryOp(_bin_op, box (lhs, rhs)) => {
                self.visit_operand(lhs, location);
                self.visit_operand(rhs, location);
            }

            /*
            Rvalue::Discriminant(place) => {
                self.visit_place(
                    place,
                    PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect),
                    location,
                );
            }
            */
            Rvalue::NullaryOp(NullOp::Box, _ty) => {
                // self.visit_ty(ty, TyContext::Location(location));
                log::error!("Box::new() is not supported!");
                unimplemented!()
            }

            _ => {
                log::error!("rvalue of this kind: {:?} is not supported!", rvalue);
                unimplemented!()
            }
        }
    }
}
