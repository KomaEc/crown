pub mod libc;
pub mod library;

use std::ops::Range;

use rustc_middle::{
    mir::{
        Body, HasLocalDecls, Location, Operand, Place, ProjectionElem, Rvalue, Terminator,
        TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::{self, FnDef};
use serde::{Deserialize, Serialize};

use self::{libc::libc_call, library::library_call};
use super::{
    boolean_system::BooleanSystem, resolve_body, BooleanLattice, ConstraintSystem, FnLocals, Infer,
    Lattice, StructFields, TypeQualifiers, Var, WithConstraintSystem,
};
use crate::{
    lattice::{HasBottom, HasTop},
    ownership::solidify::SolidifiedOwnershipSchemes,
};

pub fn fatness_analysis(
    crate_data: &common::CrateData,
    solidified: &SolidifiedOwnershipSchemes,
) -> FatnessResult {
    let mut result = FatnessResult::new_empty(crate_data);
    let mut database = BooleanSystem::new(&result.model);
    for r#fn in &crate_data.fns {
        let body = crate_data.tcx.optimized_mir(*r#fn);
        resolve_body(
            &mut database,
            &mut result,
            FatnessAnalysis { body, solidified },
            body,
            crate_data.tcx,
        );
    }
    database.greatest_model(&mut result.model);
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// [`Fatness::Arr`] ⊑ [`Fatness::Ptr`]
pub enum Fatness {
    Arr,
    Ptr,
}

impl Fatness {
    #[inline]
    pub fn is_arr(&self) -> bool {
        *self == Fatness::Arr
    }

    #[inline]
    pub fn is_ptr(&self) -> bool {
        *self == Fatness::Ptr
    }
}

impl std::fmt::Display for Fatness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fatness::Arr => write!(f, "&[]"),
            Fatness::Ptr => write!(f, "&"),
        }
    }
}

pub type FatnessResult = TypeQualifiers<Fatness>;

impl From<Fatness> for bool {
    fn from(fatness: Fatness) -> Self {
        fatness == Fatness::Ptr
    }
}

impl From<bool> for Fatness {
    fn from(b: bool) -> Self {
        if b {
            Fatness::Ptr
        } else {
            Fatness::Arr
        }
    }
}

impl HasBottom for Fatness {
    const BOTTOM: Self = Self::Arr;
}

impl HasTop for Fatness {
    const TOP: Self = Self::Ptr;
}

impl Lattice for Fatness {
    fn join(&mut self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Arr, Self::Ptr) => {
                *self = Self::Ptr;
                return true;
            }
            _ => {}
        }
        false
    }

    fn meet(&mut self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Ptr, Self::Arr) => {
                *self = Self::Arr;
                return true;
            }
            _ => {}
        }
        true
    }
}

impl BooleanLattice for Fatness {}

pub struct FatnessAnalysis<'me, 'tcx> {
    body: &'me Body<'tcx>,
    solidified: &'me SolidifiedOwnershipSchemes,
}

impl<'me, 'tcx> WithConstraintSystem for FatnessAnalysis<'me, 'tcx> {
    type DB = BooleanSystem<Fatness>;
}

impl<'me, 'tcx> Infer<'tcx> for FatnessAnalysis<'me, 'tcx> {
    fn infer_assign(
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
                let lhs = place_vars(lhs, local_decls, locals, struct_fields);
                let rhs = place_vars(rhs, local_decls, locals, struct_fields);

                // type safety
                assert_eq!(
                    lhs.end.index() - lhs.start.index(),
                    rhs.end.index() - rhs.start.index()
                );

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs);

                    let lhs_ownership = self.solidified.place_result(self.body, place);
                    if matches!(lhs_ownership.first().copied(), Some(ownership) if ownership.is_owning())
                    {
                        database.guard(rhs, lhs);
                    }
                }
                for (lhs, rhs) in lhs_rhs {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs)
                }
            }
            Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                // for cast, we process the head ptr only
                let lhs = place_vars(lhs, local_decls, locals, struct_fields);
                let rhs = place_vars(rhs, local_decls, locals, struct_fields);

                let mut lhs_rhs = lhs.zip(rhs);
                if let Some((lhs, rhs)) = lhs_rhs.next() {
                    database.guard(lhs, rhs);

                    let lhs_ownership = self.solidified.place_result(self.body, place);
                    if matches!(lhs_ownership.first().copied(), Some(ownership) if ownership.is_owning())
                    {
                        database.guard(rhs, lhs);
                    }
                }
            }
            Rvalue::Ref(_, _, rhs) | Rvalue::AddressOf(_, rhs) => {
                let lhs = place_vars(lhs, local_decls, locals, struct_fields);
                let rhs = place_vars(rhs, local_decls, locals, struct_fields);
                for (lhs, rhs) in lhs.skip(1).zip(rhs) {
                    database.guard(lhs, rhs);
                    database.guard(rhs, lhs);
                }
            }
            _ => {
                // no need. [`place_vars`] is a pure function
                // let _ = place_vars(lhs, local_decls, locals, struct_fields);
            }
        }
    }

    fn infer_terminator(
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

                            let dest = place_vars(destination, local_decls, locals, struct_fields);
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
                                let arg_vars = place_vars(&arg, local_decls, locals, struct_fields);

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
        }
    }
}

fn place_vars<'tcx>(
    place: &Place<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
) -> Range<Var> {
    let mut place_vars = Range {
        start: locals[place.local.index()],
        end: locals[place.local.index() + 1],
    };
    let mut base_ty = local_decls.local_decls()[place.local].ty;

    for projection_elem in place.projection {
        match projection_elem {
            ProjectionElem::Deref => {
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
