mod libc;
mod library;
#[cfg(test)]
mod test;

use std::ops::Range;

use rustc_middle::mir::{
    BinOp, HasLocalDecls, Location, Operand, Place, ProjectionElem, Rvalue, Terminator,
    TerminatorKind, visit::Visitor,
};
use rustc_span::source_map::Spanned;
use rustc_type_ir::TyKind;
use utils::rustc::{CallKind, RustProgram};

use crate::{
    lattice::Lattice,
    type_qualifier::foster::{
        BooleanLattice, InferCtxt, StructFields, TypeQualifiers, Var,
        constraint_system::BooleanSystem,
        mutability::{libc::libc_call, library::library_call},
    },
};
use crate::{
    lattice::{HasBottom, HasTop},
    type_qualifier::foster::constraint_system::ConstraintSystem,
};

pub fn mutability_analysis(rust_program: &RustProgram) -> MutabilityResult {
    let mut result = MutabilityResult::new_empty(rust_program);
    let mut database = BooleanSystem::new(&result.model);
    for r#fn in &rust_program.functions {
        let body = rust_program.tcx.optimized_mir(*r#fn);
        let locals = {
            let idx = result.fn_locals.0.did_idx[&body.source.def_id()];
            &result.fn_locals.0.contents[idx]
        };
        let ctxt = InferCtxt {
            local_decls: body,
            locals,
            fn_locals: &result.fn_locals,
            struct_fields: &result.struct_fields,
            tcx: rust_program.tcx,
        };

        let mut analysis = MutabilityAnalysis2 {
            ctxt,
            database: &mut database,
        };

        analysis.visit_body(body);
    }
    database.greatest_model(&mut result.model);
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        if b { Mutability::Imm } else { Mutability::Mut }
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

pub struct MutabilityAnalysis2<'infer, 'tcx, D> {
    ctxt: InferCtxt<'infer, 'tcx, D>,
    database: &'infer mut BooleanSystem<Mutability>,
}

impl<'infer, 'tcx, D: HasLocalDecls<'tcx>> Visitor<'tcx> for MutabilityAnalysis2<'infer, 'tcx, D> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, _location: Location) {
        let lhs = place;
        let rhs = rvalue;

        let InferCtxt {
            local_decls,
            locals,
            fn_locals: _,
            struct_fields,
            tcx: _,
        } = self.ctxt;
        let database = &mut self.database;

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
            Rvalue::BinaryOp(BinOp::Offset, box (ptr, _)) => {
                let dest_vars =
                    place_vars::<MutCtxt>(lhs, local_decls, locals, struct_fields, database);
                if let Some(arg) = ptr.place() {
                    let arg_vars = place_vars::<EnsureNoDeref>(
                        &arg,
                        local_decls,
                        locals,
                        struct_fields,
                        &mut (),
                    );
                    let mut dest_arg = dest_vars.zip(arg_vars);

                    if let Some((dest, arg)) = dest_arg.next() {
                        database.guard(dest, arg)
                    }
                    for (dest, arg) in dest_arg {
                        database.guard(arg, dest);
                        database.guard(dest, arg);
                    }
                }
            }
            Rvalue::Ref(_, _, rhs) | Rvalue::RawPtr(_, rhs) => {
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

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _location: Location) {
        let InferCtxt {
            local_decls,
            locals,
            fn_locals,
            struct_fields,
            tcx,
        } = self.ctxt;
        let database = &mut self.database;

        if let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind
        {
            match CallKind::new(tcx, func) {
                CallKind::FreeStanding(callee) => {
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
                        let Some(arg) = arg.node.place() else {
                            continue;
                        };
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
                }
                CallKind::Extern(ident) => {
                    libc_call(
                        destination,
                        args,
                        ident,
                        local_decls,
                        locals,
                        struct_fields,
                        database,
                    );
                }
                CallKind::Library(callee) => {
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
                }
                CallKind::Impl(..) => unimplemented!("impl method is not yet supported"),
                CallKind::Closure => unimplemented!("closure is not yet supported"),
            }
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
                base_ty = base_ty.builtin_deref(true).unwrap();
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
            ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } => {
                base_ty = base_ty.builtin_index().unwrap();
            }
            ProjectionElem::Subslice { .. } => unreachable!("unexpected subslicing"),
            ProjectionElem::OpaqueCast(_) => unreachable!("unexpected opaque cast"),
            ProjectionElem::Downcast(..) => {
                // happens when asserting nonnullness of fn ptrs
                assert!(place_vars.is_empty());
                return place_vars;
            }
            ProjectionElem::UnwrapUnsafeBinder(_) => unreachable!("unexpected UnwrapUnsafeBinder"),
            ProjectionElem::Subtype(_) => unreachable!("unexpected Subtype"),
        }
    }

    place_vars
}

pub(crate) fn conservative_call<'tcx>(
    destination: &Place<'tcx>,
    args: &[Spanned<Operand<'tcx>>],
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut BooleanSystem<Mutability>,
) {
    let dest_var = place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);

    for var in dest_var {
        database.bottom(var);
    }

    for arg in args {
        let Some(arg) = arg.node.place() else {
            continue;
        };
        let arg_vars =
            place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());
        for var in arg_vars {
            database.bottom(var);
        }
    }
}
