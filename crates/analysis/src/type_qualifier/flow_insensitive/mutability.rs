pub mod libc;
pub mod library;

use std::ops::Range;

use rustc_middle::{
    mir::{
        HasLocalDecls, Location, Operand, Place, ProjectionElem, Rvalue, Terminator, TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::{self, FnDef};
use serde::{Deserialize, Serialize};

use self::{libc::libc_call, library::library_call};
use super::{
    boolean_system::BooleanSystem, resolve_body, BooleanLattice, FnLocals, Infer, Lattice,
    StructFields, TypeQualifiers, Var, WithConstraintSystem,
};
use crate::{
    lattice::{HasBottom, HasTop},
    type_qualifier::flow_insensitive::ConstraintSystem,
};

pub fn mutability_analysis(crate_data: &common::CrateData) -> MutabilityResult {
    let mut result = MutabilityResult::new_empty(crate_data);
    let mut database = BooleanSystem::new(&result.model);
    for r#fn in &crate_data.fns {
        let body = crate_data.tcx.optimized_mir(*r#fn);
        resolve_body(
            &mut database,
            &mut result,
            MutabilityAnalysis,
            body,
            crate_data.tcx,
        );
    }
    database.greatest_model(&mut result.model);
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// [`Mutability::Mut`] ⊑ [`Mutability::Imm`]
pub enum Mutability {
    Imm,
    Mut,
}

impl Mutability {
    #[inline]
    pub fn is_mutable(&self) -> bool {
        *self == Mutability::Mut
    }

    #[inline]
    pub fn is_immutable(&self) -> bool {
        *self == Mutability::Imm
    }
}

impl std::fmt::Display for Mutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mutability::Imm => write!(f, "&read"),
            Mutability::Mut => write!(f, "&read_write"),
        }
    }
}

pub type MutabilityResult = TypeQualifiers<Mutability>;

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

impl HasBottom for Mutability {
    const BOTTOM: Self = Self::Mut;
}

impl HasTop for Mutability {
    const TOP: Self = Self::Imm;
}

impl Lattice for Mutability {
    fn join(&mut self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Mut, Self::Imm) => {
                *self = Self::Imm;
                return true;
            }
            _ => {}
        }
        false
    }

    fn meet(&mut self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Imm, Self::Mut) => {
                *self = Self::Mut;
                return true;
            }
            _ => {}
        }
        true
    }
}

impl BooleanLattice for Mutability {}

pub struct MutabilityAnalysis;

pub trait MutabilityLikeAnalysis {}

impl MutabilityLikeAnalysis for MutabilityAnalysis {}

impl<M: MutabilityLikeAnalysis> WithConstraintSystem for M {
    type DB = BooleanSystem<Mutability>;
}

impl<'tcx, M: MutabilityLikeAnalysis> Infer<'tcx> for M {
    default fn infer_assign(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        _location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFields,
        database: &mut Self::DB,
    ) {
        let lhs = place;
        let rhs = rvalue;

        match rhs {
            Rvalue::Use(Operand::Copy(rhs) | Operand::Move(rhs)) | Rvalue::CopyForDeref(rhs) => {
                let lhs = place_vars::<MutCtxt>(lhs, local_decls, locals, struct_fields, database);
                let mut rhs_deref = None;
                let rhs = place_vars::<UnknownCtxt>(
                    rhs,
                    local_decls,
                    locals,
                    struct_fields,
                    &mut rhs_deref,
                );

                // type safety
                assert_eq!(
                    lhs.end.index() - lhs.start.index(),
                    rhs.end.index() - rhs.start.index(),
                    "{:?}: {} = {:?}",
                    place,
                    local_decls.local_decls()[place.local].ty,
                    rvalue
                );

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs);
                    if let Some(rhs_deref) = rhs_deref {
                        database.guard(lhs, rhs_deref);
                    }
                }
                for (lhs, rhs) in lhs_rhs {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs)
                }
            }
            Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                // for cast, we process the head ptr only
                let lhs = place_vars::<MutCtxt>(lhs, local_decls, locals, struct_fields, database);
                let mut rhs_deref = None;
                let rhs = place_vars::<UnknownCtxt>(
                    rhs,
                    local_decls,
                    locals,
                    struct_fields,
                    &mut rhs_deref,
                );

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs);
                    if let Some(rhs_deref) = rhs_deref {
                        database.guard(lhs, rhs_deref)
                    }
                }
            }
            // We don't do this because mutable borrow does not necessarily means being mutable!
            // Rvalue::Ref(_, BorrowKind::Mut { .. }, rhs) | Rvalue::AddressOf(_, rhs) => {
            //     let lhs =
            //         place_vars::<EnsureNoDeref>(lhs, local_decls, locals, struct_fields, &mut ());
            //     let rhs = place_vars::<MutCtxt>(rhs, local_decls, locals, struct_fields, database);
            //     for (lhs, rhs) in lhs.skip(1).zip(rhs) {
            //         database.guard(lhs, rhs);
            //         database.guard(rhs, lhs);
            //     }
            // }
            Rvalue::Ref(_, _, rhs) | Rvalue::AddressOf(_, rhs) => {
                let mut lhs =
                    place_vars::<EnsureNoDeref>(lhs, local_decls, locals, struct_fields, &mut ());
                let mut rhs_deref = None;
                let rhs = place_vars::<UnknownCtxt>(
                    rhs,
                    local_decls,
                    locals,
                    struct_fields,
                    &mut rhs_deref,
                );
                let lhs_ref = lhs.next().unwrap();
                if let Some(rhs_deref) = rhs_deref {
                    database.guard(lhs_ref, rhs_deref);
                }
                for (lhs, rhs) in lhs.zip(rhs) {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs);
                }
            }
            _ => {
                let _ = place_vars::<MutCtxt>(lhs, local_decls, locals, struct_fields, database);
            }
        }
    }

    default fn infer_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        _location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocals,
        struct_fields: &StructFields,
        database: &mut Self::DB,
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
                            let mut callee_vars = fn_locals
                                .0
                                .contents_iter(&callee)
                                .take(callee_body.arg_count + 1);

                            let dest = place_vars::<MutCtxt>(
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
                                let arg_vars = place_vars::<EnsureNoDeref>(
                                    &arg,
                                    local_decls,
                                    locals,
                                    struct_fields,
                                    &mut (),
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
                            libc_call::<Self>(
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
                    library_call::<Self>(
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
            conservative_call::<Self>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            );
        }
    }
}

trait PlaceContext {
    type DerefStore;

    fn on_deref(var: Var, deref_var: &mut Self::DerefStore);
}

enum MutCtxt {}

impl PlaceContext for MutCtxt {
    // <MutabilityAnalysis as Infer<'_>>::L
    type DerefStore = BooleanSystem<Mutability>;

    fn on_deref(var: Var, database: &mut Self::DerefStore) {
        database.bottom(var);
    }
}

enum UnknownCtxt {}

impl PlaceContext for UnknownCtxt {
    type DerefStore = Option<Var>;

    fn on_deref(var: Var, deref_var: &mut Self::DerefStore) {
        assert!(deref_var.is_none());
        *deref_var = Some(var);
    }
}

enum EnsureNoDeref {}

impl PlaceContext for EnsureNoDeref {
    type DerefStore = ();

    fn on_deref(_: Var, (): &mut Self::DerefStore) {
        unreachable!()
    }
}

fn place_vars<'tcx, Ctxt: PlaceContext>(
    place: &Place<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    deref_store: &mut Ctxt::DerefStore,
) -> Range<Var> {
    let mut place_vars = Range {
        start: locals[place.local.index()],
        end: locals[place.local.index() + 1],
    };
    let mut base_ty = local_decls.local_decls()[place.local].ty;

    for projection_elem in place.projection {
        match projection_elem {
            ProjectionElem::Deref => {
                Ctxt::on_deref(place_vars.start, deref_store);
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
            ProjectionElem::OpaqueCast(_) => unreachable!("unexpected opaque cast"),
            ProjectionElem::Downcast(..) => {
                // happens when asserting nonnullness of fn ptrs
                assert!(place_vars.is_empty());
                return place_vars;
            }
        }
    }

    place_vars
}

pub(crate) fn conservative_call<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_var = place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);

    for var in dest_var {
        database.bottom(var);
    }

    for arg in args {
        let Some(arg) = arg.place() else { continue; };
        let arg_vars =
            place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());
        for var in arg_vars {
            database.bottom(var);
        }
    }
}
