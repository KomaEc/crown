//! A context-sensitive, flow-insensitive, field-based type qualifier inference framework

use std::ops::Range;

use common::data_structure::vec_array::VecArray;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, HasLocalDecls, LocalDecls, Location, NonDivergingIntrinsic,
    Place, Rvalue, Statement, StatementKind, Terminator,
};

/// A group of [`DefId`]s, with each a set of entities of concerned
trait MirGroup {}
enum StructFields {}
enum FnLocals {}

impl MirGroup for StructFields {}
impl MirGroup for FnLocals {}

struct VarGroup<Group: MirGroup, Var> {
    did_idx: FxHashMap<DefId, usize>,
    /// [`DefId`] -> entity -> [`std::ops::Range<Var>`]
    vars: VecArray<Var>,
    _group: std::marker::PhantomData<*const Group>,
}

impl<Group: MirGroup, Var: Copy> VarGroup<Group, Var> {
    #[inline]
    pub fn vars(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        let idx = self.did_idx[did];
        self.vars[idx]
            .array_windows()
            .map(|&[start, end]| Range { start, end })
    }
}

type StructFieldsVars<Var> = VarGroup<StructFields, Var>;
type FnLocalsVars<Var> = VarGroup<FnLocals, Var>;

impl<Var: Copy> StructFieldsVars<Var> {
    /// [`fields()`] returns a slice of [`Range<Var>`] that is in lock-step with [`all_fields()`]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        self.vars(did)
    }
}

impl<Var: Copy> FnLocalsVars<Var> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<Var>> + '_ {
        self.vars(did)
    }
}

trait ConstraintSystem {
    type Var;

    fn source(&mut self, var: Self::Var);

    fn sink(&mut self, var: Self::Var);

    fn guard(&mut self, guard: Self::Var, guarded: Self::Var);
}

fn infer_body<DB: ConstraintSystem>(
    body: &Body,
    locals: &[DB::Var],
    fn_locals: &FnLocalsVars<DB::Var>,
    struct_fields: &StructFieldsVars<DB::Var>,
    database: &mut DB,
) {
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        infer_basic_block(
            bb,
            bb_data,
            &body.local_decls,
            locals,
            fn_locals,
            struct_fields,
            database,
        )
    }
}

fn infer_basic_block<'tcx, DB: ConstraintSystem>(
    bb: BasicBlock,
    bb_data: &BasicBlockData<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[DB::Var],
    fn_locals: &FnLocalsVars<DB::Var>,
    struct_fields: &StructFieldsVars<DB::Var>,
    database: &mut DB,
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
        infer_statement(
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

        infer_terminator(
            terminator,
            location,
            local_decls,
            locals,
            fn_locals,
            struct_fields,
            database,
        );
    }
}

fn infer_statement<'tcx, DB: ConstraintSystem>(
    statement: &Statement<'tcx>,
    location: Location,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[DB::Var],
    struct_fields: &StructFieldsVars<DB::Var>,
    database: &mut DB,
) {
    match &statement.kind {
        StatementKind::Assign(box (place, rvalue)) => {
            infer_assign(place, rvalue, location, local_decls, locals, struct_fields, database);
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
        // | StatementKind::CopyNonOverlapping(_)
        | StatementKind::Nop => {
            unreachable!("statement {:?} is not assumed to appear", statement)
        }
    }
}

fn infer_assign<'tcx, DB: ConstraintSystem>(
    place: &Place<'tcx>,
    rvalue: &Rvalue<'tcx>,
    location: Location,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[DB::Var],
    struct_fields: &StructFieldsVars<DB::Var>,
    database: &mut DB,
) {
}

fn infer_terminator<'tcx, DB: ConstraintSystem>(
    terminator: &Terminator<'tcx>,
    location: Location,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[DB::Var],
    fn_locals: &FnLocalsVars<DB::Var>,
    struct_fields: &StructFieldsVars<DB::Var>,
    database: &mut DB,
) {
}
