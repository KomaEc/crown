pub mod libc;
pub mod library;

use std::ops::Range;

use rustc_middle::{
    mir::{
        BorrowKind, HasLocalDecls, Location, Operand, Place, ProjectionElem, Rvalue, Terminator,
        TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::{self, FnDef};

use self::{libc::libc_call, library::library_call};
use super::{
    boolean_system::BooleanSystem, AnalysisResult, BooleanLattice, FnLocalsVars, Infer, Lattice,
    StructFieldsVars, Var,
};
use crate::type_qualifier::flow_insensitive::ConstraintSystem;

pub fn mutability_analysis(crate_data: &common::CrateData) -> MutabilityResult {
    MutabilityResult::new(crate_data)
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// [`Mutability::Mut`] ⊑ [`Mutability::Imm`]
pub enum Mutability {
    Imm,
    Mut,
}

impl std::fmt::Display for Mutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mutability::Imm => write!(f, "&read"),
            Mutability::Mut => write!(f, "&read_write"),
        }
    }
}

pub type MutabilityResult = AnalysisResult<MutabilityAnalysis>;

impl From<Mutability> for bool {
    fn from(mutability: Mutability) -> Self {
        mutability == Mutability::Imm
    }
}

impl From<bool> for Mutability {
    fn from(b: bool) -> Self {
        if b {
            Mutability::Imm
        } else {
            Mutability::Mut
        }
    }
}

impl Lattice for Mutability {
    const BOTTOM: Self = Self::Mut;

    const TOP: Self = Self::Imm;
}

impl BooleanLattice for Mutability {}

pub struct MutabilityAnalysis;

impl Infer for MutabilityAnalysis {
    type L = BooleanSystem<Mutability>;

    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        _location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::L,
    ) {
        let lhs = place;
        let rhs = rvalue;

        match rhs {
            Rvalue::Use(Operand::Copy(rhs) | Operand::Move(rhs)) | Rvalue::CopyForDeref(rhs) => {
                let lhs = place_vars::<true>(lhs, local_decls, locals, struct_fields, database);
                let rhs = place_vars::<false>(rhs, local_decls, locals, struct_fields, database);

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
                let lhs = place_vars::<true>(lhs, local_decls, locals, struct_fields, database);
                let rhs = place_vars::<false>(rhs, local_decls, locals, struct_fields, database);

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs)
                }
            }
            Rvalue::Ref(_, BorrowKind::Mut { .. }, rhs) | Rvalue::AddressOf(_, rhs) => {
                let lhs = place_vars::<true>(lhs, local_decls, locals, struct_fields, database);
                let rhs = place_vars::<true>(rhs, local_decls, locals, struct_fields, database);
                for (lhs, rhs) in lhs.skip(1).zip(rhs) {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs);
                }
            }
            Rvalue::Ref(_, _, rhs) => {
                let lhs = place_vars::<true>(lhs, local_decls, locals, struct_fields, database);
                let rhs = place_vars::<false>(rhs, local_decls, locals, struct_fields, database);
                for (lhs, rhs) in lhs.skip(1).zip(rhs) {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs);
                }
            }
            _ => {
                let _ = place_vars::<true>(lhs, local_decls, locals, struct_fields, database);
            }
        }
    }

    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        _location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::L,
        tcx: TyCtxt<'tcx>,
    ) {
        if let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind
        {
            if let Some(func) = func.constant() {
                let ty = func.ty();
                let &FnDef(callee, _) = ty.kind() else { unreachable!() };
                if let Some(local_did) = callee.as_local() {
                    match tcx.hir().find_by_def_id(local_did).unwrap() {
                        // this crate
                        rustc_hir::Node::Item(_) => {
                            let callee_body = tcx.optimized_mir(callee);
                            let mut callee_vars =
                                fn_locals.vars(&callee).take(callee_body.arg_count + 1);

                            let dest = place_vars::<true>(
                                destination,
                                local_decls,
                                locals,
                                struct_fields,
                                database,
                            );
                            let ret = callee_vars.next().unwrap();

                            // type safety
                            assert_eq!(
                                dest.end.index() - dest.start.index(),
                                ret.end.index() - ret.start.index()
                            );

                            let mut dest_ret = dest.zip(ret);

                            if let Some((dest, ret)) = dest_ret.next() {
                                database.guard(dest, ret)
                            }
                            for (dest, ret) in dest_ret {
                                database.guard(ret, dest);
                                database.guard(dest, ret);
                            }

                            for (arg, param_vars) in args.iter().zip(callee_vars) {
                                let Some(arg) = arg.place() else { continue; };
                                let arg_vars = place_vars::<false>(
                                    &arg,
                                    local_decls,
                                    locals,
                                    struct_fields,
                                    database,
                                );

                                let mut param_arg = param_vars.zip(arg_vars);
                                if let Some((param, arg)) = param_arg.next() {
                                    database.guard(param, arg);
                                }
                                for (param, arg) in param_arg {
                                    database.guard(arg, param);
                                    database.guard(param, arg);
                                }
                            }

                            return;
                        }
                        // extern
                        rustc_hir::Node::ForeignItem(foreign_item) => {
                            libc_call(
                                destination,
                                args,
                                foreign_item.ident,
                                local_decls,
                                locals,
                                struct_fields,
                                database,
                            );
                            return;
                        }
                        // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                        rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                        _ => unreachable!(),
                    }
                } else {
                    library_call(
                        destination,
                        args,
                        callee,
                        local_decls,
                        locals,
                        struct_fields,
                        database,
                        tcx,
                    );
                    return;
                }
            }

            // conservative catch all
            let dest_var =
                place_vars::<true>(destination, local_decls, locals, struct_fields, database);

            for var in dest_var {
                database.bottom(var);
            }

            for arg in args {
                let Some(arg) = arg.place() else { continue; };
                let arg_vars =
                    place_vars::<false>(&arg, local_decls, locals, struct_fields, database);
                for var in arg_vars {
                    database.bottom(var);
                }
            }
        }
    }
}

fn place_vars<'tcx, const MUT: bool>(
    place: &Place<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) -> Range<Var> {
    let mut place_vars = Range {
        start: locals[place.local.index()],
        end: locals[place.local.index() + 1],
    };
    let mut base_ty = local_decls.local_decls()[place.local].ty;

    for projection_elem in place.projection {
        match projection_elem {
            ProjectionElem::Deref => {
                if MUT {
                    database.bottom(place_vars.start);
                }
                place_vars.start = place_vars.start + 1;
                base_ty = base_ty.builtin_deref(true).unwrap().ty;
            }
            ProjectionElem::Field(field, ty) => {
                assert!(place_vars.is_empty());

                match base_ty.kind() {
                    TyKind::Adt(adt_def, _) => {
                        // FIXME correctness?
                        if adt_def.is_union() {
                            return place_vars;
                        }
                        let field_vars = struct_fields
                            .fields(&adt_def.did())
                            .nth(field.index())
                            .unwrap();

                        place_vars = field_vars;

                        base_ty = ty;
                    }
                    TyKind::Tuple(..) => return place_vars,
                    _ => unreachable!(),
                }
            }
            ProjectionElem::Index(_) => {
                base_ty = base_ty.builtin_index().unwrap();
            }
            ProjectionElem::ConstantIndex { .. } => unreachable!("unexpected constant index"),
            ProjectionElem::Subslice { .. } => unreachable!("unexpected subslicing"),
            ProjectionElem::Downcast(..) => {
                // happens when asserting nonnullness of fn ptrs
                assert!(place_vars.is_empty());
                return place_vars;
            }
        }
    }

    place_vars
}
