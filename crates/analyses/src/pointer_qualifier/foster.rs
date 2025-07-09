//! Foster style type qualifier inference algorithm

mod constraint_system;
pub mod mutability;

use crate::encoding::{encode_fns, encode_structs};
use crate::pointer_qualifier::foster::constraint_system::{
    BooleanLattice, BooleanSystem, ConstraintSystem, Var,
};
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        BasicBlock, BasicBlockData, Body, HasLocalDecls, Location, NonDivergingIntrinsic, Place,
        Rvalue, Statement, StatementKind, Terminator,
    },
    ty::{Ty, TyCtxt},
};

use utils::rustc::RustProgram;
use utils::tracing;

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

pub fn resolve_body<'tcx, I, Domain>(
    database: &mut I::DB,
    result: &mut TypeQualifiers<Domain>,
    mut infer: I,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) where
    Domain: BooleanLattice,
    I: Infer<'tcx, DB = BooleanSystem<Domain>>,
    <I as WithConstraintSystem>::DB: ConstraintSystem<Domain = Domain>,
{
    let locals = {
        let idx = result.fn_locals.0.did_idx[&body.source.def_id()];
        &result.fn_locals.0.contents[idx]
    };
    infer.infer_body(
        body,
        locals,
        &result.fn_locals,
        &result.struct_fields,
        database,
        tcx,
    );
}

pub trait WithConstraintSystem {
    type DB: ConstraintSystem;
}

/// Why didn't I use a `mir::Visitor`?
pub trait Infer<'tcx>: WithConstraintSystem {
    fn infer_assign(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFields,
        database: &mut Self::DB,
    );

    fn infer_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocals,
        struct_fields: &StructFields,
        database: &mut Self::DB,
        tcx: TyCtxt<'tcx>,
    );

    fn infer_body(
        &mut self,
        body: &Body<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocals,
        struct_fields: &StructFields,
        database: &mut Self::DB,
        tcx: TyCtxt<'tcx>,
    ) {
        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            self.infer_basic_block(
                bb,
                bb_data,
                &body.local_decls,
                locals,
                fn_locals,
                struct_fields,
                database,
                tcx,
            )
        }
    }

    fn infer_basic_block(
        &mut self,
        bb: BasicBlock,
        bb_data: &BasicBlockData<'tcx>,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocals,
        struct_fields: &StructFields,
        database: &mut Self::DB,
        tcx: TyCtxt<'tcx>,
    ) {
        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = bb_data;

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.infer_statement(
                statement,
                location,
                local_decls,
                locals,
                struct_fields,
                database,
            );
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };

            self.infer_terminator(
                terminator,
                location,
                local_decls,
                locals,
                fn_locals,
                struct_fields,
                database,
                tcx,
            );
        }
    }

    fn infer_statement(
        &mut self,
        statement: &Statement<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFields,
        database: &mut Self::DB,
    ) {
        tracing::debug!("infering statement {:?}", statement);
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.infer_assign(
                    place,
                    rvalue,
                    location,
                    local_decls,
                    locals,
                    struct_fields,
                    database,
                );
            }
            StatementKind::SetDiscriminant { .. } => {
                tracing::debug!("ignoring SetDiscriminant statement {:?}", statement)
            }
            StatementKind::Deinit(..) => {
                tracing::debug!("ignoring Deinit statement {:?}", statement)
            }
            StatementKind::Intrinsic(box intrinsic) => {
                assert!(matches!(intrinsic, NonDivergingIntrinsic::Assume(..)))
            }
            StatementKind::PlaceMention(..) => {
                tracing::debug!("ignoring PlaceMention statement {:?}", statement)
            }
            StatementKind::StorageLive(_) | StatementKind::StorageDead(_) => {
                tracing::debug!("ignoring storage statement {:?}", statement)
            }
            StatementKind::ConstEvalCounter
            | StatementKind::AscribeUserType(_, _)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            | StatementKind::Nop
            | StatementKind::BackwardIncompatibleDropHint { .. } => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }
}
