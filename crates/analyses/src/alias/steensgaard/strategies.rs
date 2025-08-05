use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Body, Operand, Place, ProjectionElem},
    ty::{TyCtxt, TyKind},
};
use rustc_span::source_map::Spanned;

use crate::alias::steensgaard::{
    AbstractLocation, ConstraintGeneration, FnLocals, StructFields, constraint::Constraint,
    location::PlaceLocation,
};

pub trait FieldStrategy {
    type StructFields;

    fn place_location<'tcx>(
        place: Place<'tcx>,
        body: &Body<'tcx>,
        struct_fields: &Self::StructFields,
        fn_locals: &FnLocals,
        tcx: TyCtxt<'tcx>,
    ) -> Option<PlaceLocation>;
}

pub enum FieldInsensitive {}

impl FieldStrategy for FieldInsensitive {
    type StructFields = ();

    fn place_location<'tcx>(
        place: Place<'tcx>,
        body: &Body<'tcx>,
        (): &Self::StructFields,
        fn_locals: &FnLocals,
        _: TyCtxt<'tcx>,
    ) -> Option<PlaceLocation> {
        let loc = fn_locals.memory_location(&body.source.def_id(), place.local.as_usize());
        if !place.is_indirect() {
            Some(PlaceLocation::Plain(loc))
        } else {
            Some(PlaceLocation::Deref(loc))
        }
    }
}

/// Field-based strategy with pointer fields as the only source of points-to target
pub enum FieldBased {}

impl FieldStrategy for FieldBased {
    type StructFields = StructFields;

    fn place_location<'tcx>(
        place: Place<'tcx>,
        body: &Body<'tcx>,
        struct_fields: &Self::StructFields,
        fn_locals: &FnLocals,
        tcx: TyCtxt<'tcx>,
    ) -> Option<PlaceLocation> {
        let mut place = place.as_ref();

        // peel off all index projections
        for (place_base, proj_elem) in place.iter_projections().rev() {
            match proj_elem {
                ProjectionElem::Index(..)
                | ProjectionElem::ConstantIndex { .. }
                | ProjectionElem::Subslice { .. }
                | ProjectionElem::Downcast(..) => place = place_base,
                _ => break,
            }
        }

        if let Some((struct_place, ProjectionElem::Field(field, _))) = place.last_projection() {
            let struct_ty = struct_place.ty(body, tcx).ty;
            let TyKind::Adt(adt_def, _) = struct_ty.kind() else {
                unreachable!()
            };
            if !struct_fields.0.did_idx.contains_key(&adt_def.did()) {
                return None;
            }
            let loc = struct_fields.memory_location(&adt_def.did(), field.index());
            return Some(PlaceLocation::Plain(loc));
        }

        assert!(place.local_or_deref_local().is_some());
        let loc = fn_locals.memory_location(&body.source.def_id(), place.local.as_usize());
        if place.as_local().is_some() {
            return Some(PlaceLocation::Plain(loc));
        } else {
            return Some(PlaceLocation::Deref(loc));
        }
    }
}

pub trait DeallocArgStrategy: Sized {
    type Arg;

    fn handle_dealloc_arg<'cg, 'tcx, F: FieldStrategy, I: InterProceduralStrategy>(
        cg: &mut ConstraintGeneration<'cg, 'tcx, F, Self, I>,
        dealloc_arg: &Operand<'tcx>,
    );
}

pub enum MergeDeallocArg {}
pub enum NopDeallocArg {}

impl DeallocArgStrategy for MergeDeallocArg {
    type Arg = AbstractLocation;

    fn handle_dealloc_arg<'cg, 'tcx, F: FieldStrategy, I: InterProceduralStrategy>(
        cg: &mut ConstraintGeneration<'cg, 'tcx, F, Self, I>,
        dealloc_arg: &Operand<'tcx>,
    ) {
        let Some(dealloc_arg) = dealloc_arg.place() else {
            return;
        };
        let ty = dealloc_arg.ty(cg.body, cg.tcx).ty;
        assert!(ty.is_raw_ptr() || ty.is_ref());
        let Some(arg_loc) = cg.place_location(dealloc_arg) else {
            return;
        };
        let PlaceLocation::Plain(arg_loc) = arg_loc else {
            unreachable!("argument operand contains derefs")
        };
        let param_loc = cg.steensgaard.dealloc_arg;
        let constraint_idx = cg.constraints.len();
        cg.constraints.push(Constraint::assign(param_loc, arg_loc));
        cg.resolve_assign(param_loc, arg_loc, constraint_idx)
    }
}

impl DeallocArgStrategy for NopDeallocArg {
    type Arg = ();

    fn handle_dealloc_arg<'cg, 'tcx, F: FieldStrategy, I: InterProceduralStrategy>(
        _: &mut ConstraintGeneration<'cg, 'tcx, F, Self, I>,
        _: &Operand<'tcx>,
    ) {
    }
}

pub trait InterProceduralStrategy: Sized {
    fn handle_extern_call<'cg, 'tcx, F: FieldStrategy, D: DeallocArgStrategy>(
        cg: &mut ConstraintGeneration<'cg, 'tcx, F, D, Self>,
        destination: &Place<'tcx>,
        args: &[Spanned<Operand<'tcx>>],
    ) {
        let dest_ty = destination.ty(cg.body, cg.tcx).ty;
        if !dest_ty.is_raw_ptr() && !dest_ty.is_ref() {
            return;
        }
        let Some(dest_loc) = cg.place_location(*destination) else {
            return;
        };
        let PlaceLocation::Plain(dest_loc) = dest_loc else {
            unreachable!("destination place contains derefs")
        };

        for arg in args.iter() {
            let Some(place) = arg.node.place() else {
                continue;
            };
            let place_ty = place.ty(cg.body, cg.tcx).ty;
            if !place_ty.is_raw_ptr() && !place_ty.is_ref() {
                continue;
            }
            let Some(arg_loc) = cg.place_location(place) else {
                continue;
            };
            let PlaceLocation::Plain(arg_loc) = arg_loc else {
                unreachable!("argument operand contains derefs")
            };
            let constraint_idx = cg.constraints.len();
            cg.constraints.push(Constraint::assign(dest_loc, arg_loc));
            cg.resolve_assign(dest_loc, arg_loc, constraint_idx)
        }
    }

    fn handle_boundary<'cg, 'tcx, F: FieldStrategy, D: DeallocArgStrategy>(
        cg: &mut ConstraintGeneration<'cg, 'tcx, F, D, Self>,
        callee_did: DefId,
        destination: &Place<'tcx>,
        args: &[Spanned<Operand<'tcx>>],
    ) {
        for (idx, arg) in args.iter().enumerate() {
            let Some(place) = arg.node.place() else {
                continue;
            };
            let place_ty = place.ty(cg.body, cg.tcx).ty;
            if !place_ty.is_raw_ptr() && !place_ty.is_ref() {
                continue;
            }
            let Some(arg_loc) = cg.place_location(place) else {
                continue;
            };
            // let param_loc = cg.steensgaard.fn_locals.locations
            //     [cg.steensgaard.fn_locals.did_idx[&callee_did]][idx + 1];
            let param_loc = cg
                .steensgaard
                .fn_locals
                .memory_location(&callee_did, idx + 1);

            let PlaceLocation::Plain(arg_loc) = arg_loc else {
                unreachable!("argument operand contains derefs")
            };
            let constraint_idx = cg.constraints.len();
            cg.constraints.push(Constraint::assign(param_loc, arg_loc));
            cg.resolve_assign(param_loc, arg_loc, constraint_idx)
        }

        let dest_ty = destination.ty(cg.body, cg.tcx).ty;
        if !dest_ty.is_raw_ptr() && !dest_ty.is_ref() {
            return;
        }

        let Some(dest_loc) = cg.place_location(*destination) else {
            return;
        };
        let PlaceLocation::Plain(dest_loc) = dest_loc else {
            unreachable!("destination place contains derefs")
        };
        // let ret_loc =
        //     cg.steensgaard.fn_locals.locations[cg.steensgaard.fn_locals.did_idx[&callee_did]][0];
        let ret_loc = cg.steensgaard.fn_locals.memory_location(&callee_did, 0);
        let constraint_idx = cg.constraints.len();
        cg.constraints.push(Constraint::assign(dest_loc, ret_loc));
        cg.resolve_assign(dest_loc, ret_loc, constraint_idx);
    }
}

pub enum InterProcedural {}

impl InterProceduralStrategy for InterProcedural {}

pub enum IntraProcedural {}
impl InterProceduralStrategy for IntraProcedural {
    fn handle_extern_call<'cg, 'tcx, F: FieldStrategy, D: DeallocArgStrategy>(
        _: &mut ConstraintGeneration<'cg, 'tcx, F, D, Self>,
        _: &Place<'tcx>,
        _: &[Spanned<Operand<'tcx>>],
    ) {
    }
    fn handle_boundary<'cg, 'tcx, F: FieldStrategy, D: DeallocArgStrategy>(
        _: &mut ConstraintGeneration<'cg, 'tcx, F, D, Self>,
        _: DefId,
        _: &Place<'tcx>,
        _: &[Spanned<Operand<'tcx>>],
    ) {
    }
}
