//! Foster style flow-insensitive type qualifier inference algorithm

mod constraint_system;
pub mod mutability;
pub mod pp;

use crate::encoding::{encode_fns, encode_structs};
use crate::type_qualifier::foster::constraint_system::{BooleanLattice, ConstraintSystem, Var};
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::ty::{Ty, TyCtxt};

use utils::rustc::RustProgram;

pub type StructFields = crate::encoding::StructFields<Var>;
pub type FnLocals = crate::encoding::FnLocals<Var>;

pub struct TypeQualifiers<Qualifier> {
    struct_fields: StructFields,
    fn_locals: FnLocals,
    model: IndexVec<Var, Qualifier>,
}

fn count_ptr(mut ty: Ty) -> usize {
    let mut cnt = 0;
    loop {
        if let Some(inner_ty) = ty.builtin_deref(true) {
            cnt += 1;
            ty = inner_ty;
            continue;
        }
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }
        break cnt;
    }
}

impl<Domain> TypeQualifiers<Domain>
where
    Domain: BooleanLattice,
{
    /// construct a new `TypeQualifiers` instance with no constraints added
    pub fn new_empty(rust_program: &RustProgram) -> Self {
        let tcx = rust_program.tcx;
        let fns = &rust_program.functions[..];
        let structs = &rust_program.structs[..];

        let mut model = IndexVec::new();
        // not necessary, but need initialization anyway
        model.push(Domain::TOP);
        model.push(Domain::BOTTOM);
        let next: Var = model.next_index();

        let (struct_fields, next) = encode_structs(next, structs, tcx, |field_ty| {
            let num_ptrs = count_ptr(field_ty);
            model.extend(std::iter::repeat(Domain::BOTTOM).take(num_ptrs));
            num_ptrs
        });
        let (fn_locals, _) = encode_fns(next, fns, tcx, |local_ty| {
            let num_ptrs = count_ptr(local_ty);
            model.extend(std::iter::repeat(Domain::BOTTOM).take(num_ptrs));
            num_ptrs
        });

        Self {
            struct_fields,
            fn_locals,
            model,
        }
    }
}

impl<Qualifier> TypeQualifiers<Qualifier> {
    pub fn function_facts(&self, did: &DefId, tcx: TyCtxt) -> impl Iterator<Item = &[Qualifier]> {
        let body = &*tcx
            .mir_drops_elaborated_and_const_checked(did.expect_local())
            .borrow();
        self.fn_locals
            .locals(did)
            .take(body.arg_count + 1)
            .map(|vars| &self.model[vars])
    }

    pub fn function_body_facts(&self, did: &DefId) -> impl Iterator<Item = &[Qualifier]> {
        self.fn_locals.locals(did).map(|vars| &self.model[vars])
    }

    pub fn struct_facts(&self, did: &DefId) -> impl Iterator<Item = &[Qualifier]> {
        self.struct_fields.fields(did).map(|vars| &self.model[vars])
    }
}

pub struct InferCtxt<'infer, 'tcx, D> {
    local_decls: &'infer D,
    locals: &'infer [Var],
    fn_locals: &'infer FnLocals,
    struct_fields: &'infer StructFields,
    tcx: TyCtxt<'tcx>,
}
