use std::ops::Range;

use rustc_middle::{
    mir::{
        BorrowKind, HasLocalDecls, Location, Operand, Place, ProjectionElem, Rvalue, Terminator,
    },
    ty::TyCtxt,
};

use super::framework::{
    BooleanLatticeSystem, ConstraintSystem, FnLocalsVars, Infer, StructFieldsVars, Var,
};

/// [`Write`] ⊑ [`Read`]
pub enum MutabilityLattice {
    Read,
    Write,
}

pub struct Mutability;

impl Infer for Mutability {
    type C = BooleanLatticeSystem;

    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::C,
        tcx: TyCtxt<'tcx>,
    ) {
        let lhs = place;
        let rhs = rvalue;

        match rhs {
            Rvalue::Use(Operand::Copy(rhs) | Operand::Move(rhs)) | Rvalue::CopyForDeref(rhs) => {
                let lhs = place_var::<true>(lhs, local_decls, locals, struct_fields, database, tcx);
                let rhs =
                    place_var::<false>(rhs, local_decls, locals, struct_fields, database, tcx);

                // type safety
                assert_eq!(
                    lhs.end.index() - lhs.start.index(),
                    rhs.end.index() - rhs.start.index()
                );

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs)
                }
                for (lhs, rhs) in lhs_rhs {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs)
                }
            }
            Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                // for cast, we process the head ptr only
                let lhs = place_var::<true>(lhs, local_decls, locals, struct_fields, database, tcx);
                let rhs =
                    place_var::<false>(rhs, local_decls, locals, struct_fields, database, tcx);

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs)
                }
            }
            Rvalue::Ref(_, BorrowKind::Mut { .. }, rhs)
            | Rvalue::AddressOf(rustc_ast::Mutability::Mut, rhs) => {}
            Rvalue::Ref(_, _, rhs) | Rvalue::AddressOf(_, rhs) => {}
            _ => {}
        }
    }

    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::C,
        tcx: TyCtxt<'tcx>,
    ) {
    }
}

fn place_var<'tcx, const MUT: bool>(
    place: &Place<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <Mutability as Infer>::C,
    tcx: TyCtxt<'tcx>,
) -> Range<Var> {
    let mut place_var = Range {
        start: locals[place.local.index()],
        end: locals[place.local.index() + 1],
    };
    let mut base_ty = place.ty(local_decls, tcx).ty;

    for projection_elem in place.projection {
        match projection_elem {
            ProjectionElem::Deref => {
                if MUT {
                    database.source(place_var.start);
                }
                place_var.start = place_var.start + 1;
                base_ty = base_ty.builtin_deref(true).unwrap().ty;
            }
            ProjectionElem::Field(field, ty) => {
                // FIXME
                // database.source(lhs_var.start);
                assert!(place_var.is_empty());

                let adt_def = ty.ty_adt_def().unwrap();
                let field_vars = struct_fields
                    .fields(&adt_def.did())
                    .nth(field.index())
                    .unwrap();

                place_var = field_vars;

                base_ty = ty;
            }
            ProjectionElem::Index(_) => {
                base_ty = base_ty.builtin_index().unwrap();
            }
            ProjectionElem::ConstantIndex { .. } => unreachable!("unexpected constant index"),
            ProjectionElem::Subslice { .. } => unreachable!("unexpected subslicing"),
            ProjectionElem::Downcast(..) => unreachable!("unexpected downcasting"),
        }
    }

    place_var
}
