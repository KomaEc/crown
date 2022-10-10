//! A context-sensitive, flow-insensitive, field-based type qualifier inference framework

use std::ops::Range;

use common::data_structure::vec_array::VecArray;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{
        BasicBlock, BasicBlockData, Body, HasLocalDecls, Location, NonDivergingIntrinsic, Place,
        Rvalue, Statement, StatementKind, Terminator,
    },
    ty::TyCtxt,
};

common::macros::newtype_index! {
    pub struct Var {
        DEBUG_FORMAT = "{}"
    }
}

/// A group of [`DefId`]s, with each a set of entities of concerned
pub trait MirGroup {}
pub enum StructFields {}
pub enum FnLocals {}

impl MirGroup for StructFields {}
impl MirGroup for FnLocals {}

pub struct VarGroup<Group: MirGroup> {
    did_idx: FxHashMap<DefId, usize>,
    /// [`DefId`] -> entity -> [`std::ops::Range<Var>`]
    vars: VecArray<Var>,
    _group: std::marker::PhantomData<*const Group>,
}

impl<Group: MirGroup> VarGroup<Group> {
    #[inline]
    pub fn vars(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        let idx = self.did_idx[did];
        self.vars[idx]
            .array_windows()
            .map(|&[start, end]| Range { start, end })
    }
}

pub type StructFieldsVars = VarGroup<StructFields>;
pub type FnLocalsVars = VarGroup<FnLocals>;

impl StructFieldsVars {
    /// [`fields()`] returns a slice of [`Range<Var>`] that is in lock-step with [`all_fields()`]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        self.vars(did)
    }
}

impl FnLocalsVars {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        self.vars(did)
    }
}

pub trait ConstraintSystem {
    fn source(&mut self, var: Var);

    fn sink(&mut self, var: Var);

    fn guard(&mut self, guard: Var, guarded: Var);
}

pub trait Infer {
    type C: ConstraintSystem;
    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::C,
        tcx: TyCtxt<'tcx>,
    );

    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::C,
        tcx: TyCtxt<'tcx>,
    );
}

fn infer_body<'tcx, I: Infer>(
    body: &Body<'tcx>,
    locals: &[Var],
    fn_locals: &FnLocalsVars,
    struct_fields: &StructFieldsVars,
    database: &mut I::C,
    tcx: TyCtxt<'tcx>,
) {
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        infer_basic_block::<I>(
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

fn infer_basic_block<'tcx, I: Infer>(
    bb: BasicBlock,
    bb_data: &BasicBlockData<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    fn_locals: &FnLocalsVars,
    struct_fields: &StructFieldsVars,
    database: &mut I::C,
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
        infer_statement::<I>(
            statement,
            location,
            local_decls,
            locals,
            struct_fields,
            database,
            tcx,
        );
        index += 1;
    }

    if let Some(terminator) = terminator {
        let location = Location {
            block: bb,
            statement_index: index,
        };

        I::infer_terminator(
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

fn infer_statement<'tcx, I: Infer>(
    statement: &Statement<'tcx>,
    location: Location,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut I::C,
    tcx: TyCtxt<'tcx>,
) {
    match &statement.kind {
        StatementKind::Assign(box (place, rvalue)) => {
            I::infer_assign(
                place,
                rvalue,
                location,
                local_decls,
                locals,
                struct_fields,
                database,
                tcx,
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
        StatementKind::AscribeUserType(_, _)
        | StatementKind::StorageLive(_)
        | StatementKind::StorageDead(_)
        | StatementKind::Retag(_, _)
        | StatementKind::FakeRead(_)
        | StatementKind::Coverage(_)
        | StatementKind::Nop => {
            unreachable!("statement {:?} is not assumed to appear", statement)
        }
    }
}

pub struct BooleanLatticeSystem;

impl ConstraintSystem for BooleanLatticeSystem {
    fn source(&mut self, var: Var) {
        todo!()
    }

    fn sink(&mut self, var: Var) {
        todo!()
    }

    fn guard(&mut self, guard: Var, guarded: Var) {
        todo!()
    }
}
