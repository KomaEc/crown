//! A context-sensitive, flow-insensitive, field-based type qualifier inference framework

pub mod boolean_system;
pub mod fatness;
// pub mod local_mutability;
pub mod mutability;

use std::ops::Range;

use common::{
    data_structure::vec_vec::VecVec,
    discretization::{self, Discretization},
};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        BasicBlock, BasicBlockData, Body, HasLocalDecls, Local, Location, NonDivergingIntrinsic,
        Place, Rvalue, Statement, StatementKind, Terminator,
    },
    ty::{Ty, TyCtxt},
};
use rustc_type_ir::TyKind;

use self::boolean_system::BooleanSystem;
use crate::lattice::Lattice;

pub type StructFields = discretization::StructFields<Var>;
pub type FnLocals = discretization::FnLocals<Var>;

pub struct TypeQualifiers<Qualifier> {
    struct_fields: StructFields,
    fn_locals: FnLocals,
    model: IndexVec<Var, Qualifier>,
}

fn display_value<Value: std::fmt::Display>(value: &[Value]) -> String {
    value
        .iter()
        .map(|value| format!("{value}"))
        .collect::<Vec<_>>()
        .join(" ")
}

impl<Qualifier> TypeQualifiers<Qualifier> {
    pub fn new(
        struct_fields: StructFields,
        fn_locals: FnLocals,
        model: IndexVec<Var, Qualifier>,
    ) -> Self {
        Self {
            struct_fields,
            fn_locals,
            model,
        }
    }

    pub fn fn_results<'me>(&'me self, r#fn: &DefId) -> FnResult<'me, Qualifier> {
        let locals = &self.fn_locals.0.contents[self.fn_locals.0.did_idx[r#fn]];
        FnResult {
            locals,
            model: &self.model,
        }
    }

    pub fn struct_results(&self, r#struct: &DefId) -> impl Iterator<Item = &[Qualifier]> {
        self.struct_fields
            .fields(r#struct)
            .map(|Range { start, end }| &self.model.raw[start.index()..end.index()])
    }

    pub fn struct_field_result(&self, r#struct: &DefId, f: usize) -> &[Qualifier] {
        let Range { start, end } = self.struct_fields.field(r#struct, f);
        &self.model.raw[start.index()..end.index()]
    }

    pub fn fn_sig(&self, r#fn: &DefId, tcx: TyCtxt) -> impl Iterator<Item = &[Qualifier]> {
        let fn_result = self.fn_results(r#fn);
        let body = tcx.optimized_mir(*r#fn);
        fn_result.results().take(body.arg_count + 1)
    }

    fn print_fn_sigs(&self, tcx: TyCtxt, fns: &[DefId])
    where
        Qualifier: std::fmt::Display,
    {
        for did in fns {
            let mut fn_sig = self.fn_sig(did, tcx);
            let ret = fn_sig.next().unwrap();
            let ret = display_value(ret);
            let args = fn_sig.map(display_value).collect::<Vec<_>>().join(", ");

            let fn_path = tcx.def_path_str(*did);
            println!("{fn_path}: ({args}) -> {ret}")
        }
    }

    fn print_struct_sigs(&self, tcx: TyCtxt, structs: &[DefId])
    where
        Qualifier: std::fmt::Display,
    {
        for did in structs {
            let struct_results = self.struct_results(did);
            let struct_ty = tcx.type_of(*did);
            let TyKind::Adt(adt_def, _) = struct_ty.kind() else { unreachable!() };
            println!("{} {{", tcx.def_path_str(*did));
            for (field_def, qualifiers) in adt_def.all_fields().zip(struct_results) {
                println!(
                    "  {}: {},",
                    field_def.ident(tcx).as_str(),
                    display_value(qualifiers)
                );
            }
            println!("}}");
        }
    }

    pub fn print_results(&self, crate_data: &common::CrateData)
    where
        Qualifier: std::fmt::Display,
    {
        self.print_struct_sigs(crate_data.tcx, &crate_data.structs);
        self.print_fn_sigs(crate_data.tcx, &crate_data.fns);
    }

    pub fn place_result<'tcx>(&self, body: &Body<'tcx>, place: &Place<'tcx>) -> &[Qualifier] {
        let mut ptr_kinds = &self
            .fn_results(&body.source.def_id())
            .local_result(place.local)[..];
        let mut ptr_kinds_index = 0;
        let mut ty = body.local_decls[place.local].ty;
        for proj in place.projection {
            match proj {
                rustc_middle::mir::ProjectionElem::Deref => {
                    ptr_kinds_index += 1;
                    ty = ty.builtin_deref(true).unwrap().ty;
                }
                rustc_middle::mir::ProjectionElem::Field(f, field_ty) => {
                    assert_eq!(ptr_kinds_index, ptr_kinds.len());
                    let adt_def = ty.ty_adt_def().unwrap();
                    ptr_kinds = self.struct_field_result(&adt_def.did(), f.index());
                    ptr_kinds_index = 0;
                    ty = field_ty;
                }
                rustc_middle::mir::ProjectionElem::Index(_) => {
                    ty = ty.builtin_index().unwrap();
                }
                _ => unreachable!(),
            }
        }

        &ptr_kinds[ptr_kinds_index..]
    }
}

#[derive(Clone, Copy)]
pub struct FnResult<'me, Domain> {
    locals: &'me [Var],
    model: &'me IndexVec<Var, Domain>,
}

impl<'me, Domain> FnResult<'me, Domain> {
    pub fn results(self) -> impl Iterator<Item = &'me [Domain]> {
        self.locals
            .array_windows()
            .map(|&[start, end]| &self.model.raw[start.index()..end.index()])
    }

    pub fn local_result(self, local: Local) -> &'me [Domain] {
        let (start, end) = (self.locals[local.index()], self.locals[local.index() + 1]);
        &self.model.raw[start.index()..end.index()]
    }
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

// pub fn infer(crate_data: &common::CrateData)

impl<Domain> TypeQualifiers<Domain>
where
    Domain: BooleanLattice,
{
    /// construct a new `TypeQualifiers` instance with no constraints added
    pub fn new_empty(crate_data: &common::CrateData) -> Self {
        let mut model = IndexVec::new();
        // not necessary, but need initialization anyway
        model.push(Domain::TOP);
        model.push(Domain::BOTTOM);
        let mut next: Var = model.next_index();
        let tcx = crate_data.tcx;
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.structs.len());
        let mut vars =
            VecVec::with_capacity(crate_data.structs.len(), crate_data.structs.len() * 4);
        for (idx, r#struct) in crate_data.structs.iter().enumerate() {
            did_idx.insert(*r#struct, idx);
            let struct_ty = tcx.type_of(*r#struct);
            let TyKind::Adt(adt_def, substs) = struct_ty.kind() else { unreachable!() };
            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, substs);
                let ptr_count = count_ptr(field_ty);
                model.extend(std::iter::repeat(Domain::BOTTOM).take(ptr_count));
                vars.push_inner(next);
                next = next + ptr_count;
                assert_eq!(model.next_index(), next);
            }
            vars.push_inner(next);
            vars.push();
        }
        let vars = vars.done();
        let struct_fields = discretization::StructFields(Discretization {
            did_idx,
            contents: vars,
        });
        let mut did_idx = FxHashMap::default();
        did_idx.reserve(crate_data.fns.len());
        let mut vars = VecVec::with_capacity(crate_data.fns.len(), crate_data.fns.len() * 15);
        for (idx, r#fn) in crate_data.fns.iter().enumerate() {
            did_idx.insert(*r#fn, idx);
            let body = tcx.optimized_mir(*r#fn);
            for local_decl in &body.local_decls {
                let ty = local_decl.ty;
                let ptr_count = count_ptr(ty);
                model.extend(std::iter::repeat(Domain::BOTTOM).take(ptr_count));
                vars.push_inner(next);
                next = next + ptr_count;
                assert_eq!(model.next_index(), next);
            }
            vars.push_inner(next);
            vars.push();
        }
        let vars = vars.done();
        let fn_locals = discretization::FnLocals(Discretization {
            did_idx,
            contents: vars,
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

// pub trait Lattice: Clone {
//     const BOTTOM: Self;
//     const TOP: Self;
// }

pub trait BooleanLattice: From<bool> + Into<bool> + Copy + Lattice {}

common::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct Var {
        // DEBUG_FORMAT = "{}"
    }
}

pub trait ConstraintSystem {
    type Domain: Lattice;

    fn top(&mut self, var: Var);

    fn bottom(&mut self, var: Var);

    /// [`guard`] -> [`guarded`] or [`guard`] ⊒ [`guarded`]
    fn guard(&mut self, guard: Var, guarded: Var);
}

pub trait WithConstraintSystem {
    type DB: ConstraintSystem;
}

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
}
