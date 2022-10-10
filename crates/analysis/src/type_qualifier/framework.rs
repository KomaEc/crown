//! A context-sensitive, flow-insensitive, field-based type qualifier inference framework

pub mod boolean_system;

use std::ops::Range;

use common::data_structure::vec_array::VecArray;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        BasicBlock, BasicBlockData, Body, HasLocalDecls, Location, NonDivergingIntrinsic, Place,
        Rvalue, Statement, StatementKind, Terminator,
    },
    ty::{Ty, TyCtxt},
};
use rustc_type_ir::TyKind;

use self::boolean_system::BooleanSystem;

pub struct AnalysisResults<'tcx, I: Infer> {
    struct_fields: StructFieldsVars,
    fn_locals: FnLocalsVars,
    model: IndexVec<Var, <<I as Infer>::L as ConstraintSystem>::Domain>,
    tcx: TyCtxt<'tcx>,
}

fn count_ptr(mut ty: Ty) -> usize {
    let mut cnt = 0;
    loop {
        if let Some(ty_mut) = ty.builtin_deref(true) {
            cnt += 1;
            ty = ty_mut.ty;
            continue;
        }
        if let Some(inner_ty) = ty.builtin_index() {
            ty = inner_ty;
            continue;
        }
        break cnt;
    }
}

impl<'tcx, Domain, I: Infer> AnalysisResults<'tcx, I>
where
    <I as Infer>::L: ConstraintSystem<Domain = Domain>,
    Domain: BooleanLattice,
    I: Infer<L = BooleanSystem<Domain>>,
{
    pub fn new(crate_data: &common::CrateData<'tcx>) -> Self {
        let mut model = IndexVec::new();
        // not necessary, but need initialization anyway
        model.push(<<I as Infer>::L as ConstraintSystem>::Domain::BOTTOM);
        model.push(<<I as Infer>::L as ConstraintSystem>::Domain::TOP);
        let mut next: Var = model.next_index();
        let tcx = crate_data.tcx;
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut vars =
            VecArray::with_capacity(crate_data.structs.len(), crate_data.structs.len() * 4);
        for (idx, r#struct) in crate_data.structs.iter().enumerate() {
            did_idx.insert(*r#struct, idx);
            let struct_ty = tcx.type_of(*r#struct);
            let TyKind::Adt(adt_def, substs) = struct_ty.kind() else { unreachable!() };
            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, substs);
                let ptr_count = count_ptr(field_ty);
                model.extend(
                    std::iter::repeat(<<I as Infer>::L as ConstraintSystem>::Domain::BOTTOM)
                        .take(ptr_count),
                );
                vars.add_item_to_array(next);
                next = next + ptr_count;
                assert_eq!(model.next_index(), next);
            }
            vars.add_item_to_array(next);
            vars.done_with_array();
        }
        let vars = vars.done();
        let struct_fields = StructFieldsVars {
            did_idx,
            vars,
            _group: std::marker::PhantomData,
        };
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.fns.len());
        let mut vars = VecArray::with_capacity(crate_data.fns.len(), crate_data.fns.len() * 15);
        for (idx, r#fn) in crate_data.fns.iter().enumerate() {
            did_idx.insert(*r#fn, idx);
            let body = tcx.optimized_mir(*r#fn);
            for local_decl in &body.local_decls {
                let ty = local_decl.ty;
                let ptr_count = count_ptr(ty);
                model.extend(
                    std::iter::repeat(<<I as Infer>::L as ConstraintSystem>::Domain::BOTTOM)
                        .take(ptr_count),
                );
                vars.add_item_to_array(next);
                next = next + ptr_count;
                assert_eq!(model.next_index(), next);
            }
            vars.add_item_to_array(next);
            vars.done_with_array();
        }
        let vars = vars.done();
        let fn_locals = FnLocalsVars {
            did_idx,
            vars,
            _group: std::marker::PhantomData,
        };

        let mut database = BooleanSystem::new(&model);

        // FIXME context sensitive
        for r#fn in &crate_data.fns {
            let body = tcx.optimized_mir(*r#fn);
            let locals = {
                let idx = fn_locals.did_idx[r#fn];
                &fn_locals.vars[idx]
            };
            infer_body::<I>(body, locals, &fn_locals, &struct_fields, &mut database, tcx);
        }

        database.greatest_model(&mut model);

        Self {
            struct_fields,
            fn_locals,
            model,
            tcx,
        }
    }
}

pub trait Lattice: Clone {
    const BOTTOM: Self;
    const TOP: Self;
}

pub trait BooleanLattice: Copy + PartialEq + Eq + From<bool> + Into<bool> + Lattice {}

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
    type Domain: Lattice;

    fn top(&mut self, var: Var);

    fn bottom(&mut self, var: Var);

    /// [`guard`] -> [`guarded`] or [`guarded`] ⊑ [`guard`]
    fn guard(&mut self, guard: Var, guarded: Var);
}

pub trait Infer {
    type L: ConstraintSystem;
    fn infer_assign<'tcx>(
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        struct_fields: &StructFieldsVars,
        database: &mut Self::L,
        tcx: TyCtxt<'tcx>,
    );

    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::L,
        tcx: TyCtxt<'tcx>,
    );
}

fn infer_body<'tcx, I: Infer>(
    body: &Body<'tcx>,
    locals: &[Var],
    fn_locals: &FnLocalsVars,
    struct_fields: &StructFieldsVars,
    database: &mut I::L,
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
    database: &mut I::L,
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
    database: &mut I::L,
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

// pub struct BooleanLatticeSystem;

// impl LatticeConstraintSystem for BooleanLatticeSystem {
//     type Lattice;

//     fn top(&mut self, var: Var) {
//         todo!()
//     }

//     fn bottom(&mut self, var: Var) {
//         todo!()
//     }

//     fn guard(&mut self, guard: Var, guarded: Var) {
//         todo!()
//     }
// }
