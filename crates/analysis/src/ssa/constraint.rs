use std::{num::NonZeroU32, ops::Range};

use common::data_structure::vec_vec::VecVec;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{LocalDecl, LocalInfo},
    ty::TyKind,
};

use super::consume::Voidable;
use crate::{ptr::Measurable, struct_ctxt::StructCtxt, CrateCtxt};

pub mod infer;
// pub mod prune;

common::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct Var {
        // DEBUG_FORMAT = "{}"// "𝕆({})"
    }
}

impl std::fmt::Display for Var {
    // \mathbb{O}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("𝕆({})", self.as_u32()))
    }
}

impl Var {
    pub const INVALID: Self = Var::from_u32(0);
    pub const MIN: Self = Var::from_u32(1);

    pub fn into_lit(self) -> i32 {
        self.as_u32() as i32
    }
}

impl std::ops::Add<u32> for Var {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Var::from_u32(self.as_u32() + rhs)
    }
}

impl std::ops::AddAssign<u32> for Var {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs
    }
}

#[inline]
pub fn local_has_non_zero_measure<'tcx>(
    local_decl: &LocalDecl,
    measurable: impl Measurable<'tcx>,
) -> bool {
    local_measure(local_decl, measurable).is_some()
}

/// test whether a local might be owning
#[inline]
pub fn local_measure<'tcx>(
    local_decl: &LocalDecl,
    measurable: impl Measurable<'tcx>,
) -> Option<NonZeroU32> {
    let ty = local_decl.ty;
    let measure = measurable.measure(ty, 0);
    (!matches!(local_decl.local_info, Some(box LocalInfo::DerefTemp)))
        .then(|| NonZeroU32::new(measure))
        .flatten()
}

#[inline]
pub fn initialize_local<'tcx>(
    local_decl: &LocalDecl,
    gen: &mut Gen,
    database: &mut impl Database,
    measurable: impl Measurable<'tcx>,
) -> Option<Range<Var>> {
    local_measure(local_decl, measurable).map(|measure| database.new_vars(gen, measure.get()))
}

pub struct GlobalAssumptions {
    struct_fields: VecVec<Var>,
}

impl GlobalAssumptions {
    pub fn new<'tcx>(
        crate_ctxt: &CrateCtxt<'tcx>,
        gen: &mut Gen,
        database: &mut impl Database,
    ) -> Self {
        let CrateCtxt {
            tcx,
            ref struct_ctxt,
            ..
        } = *crate_ctxt;
        let mut struct_fields = VecVec::with_indices_capacity(struct_ctxt.post_order.len());

        for &did in &struct_ctxt.post_order {
            let ty = tcx.type_of(did);
            let TyKind::Adt(adt_def, subst) = ty.kind() else { unreachable!() };
            struct_fields.push_inner(gen.next());
            for field_def in adt_def.all_fields() {
                let field_ty = field_def.ty(tcx, subst);
                let ptr_depth = struct_ctxt.measure_ptr(field_ty);
                database.new_vars(gen, ptr_depth);
                struct_fields.push_inner(gen.next());
            }
            struct_fields.push();
        }

        let struct_fields = struct_fields.done();

        GlobalAssumptions { struct_fields }
    }

    pub fn fields<'a, 'tcx>(
        &'a self,
        struct_ctxt: &'a StructCtxt<'tcx>,
        did: &DefId,
    ) -> impl Iterator<Item = Range<Var>> + 'a {
        let idx = struct_ctxt.did_idx(did);
        self.struct_fields[idx]
            .array_windows()
            .map(|&[start, end]| start..end)
    }

    pub fn show(&self, struct_ctxt: &StructCtxt) {
        for (&did, fields) in
            itertools::izip!(struct_ctxt.post_order.iter(), self.struct_fields.iter())
        {
            let mut index = 0;
            println!("{:?}: {{", did);
            fields.array_windows().for_each(|&[start, end]| {
                println!("  {index}: {:?}", start..end);
                index += 1;
            });
            println!("}}");
        }
    }
}

impl Voidable for Range<Var> {
    const VOID: Self = Range {
        start: Var::INVALID,
        end: Var::INVALID,
    };

    #[inline]
    fn is_void(&self) -> bool {
        self.start == Var::INVALID && self.end == Var::INVALID
    }
}

/// A generator for [Var]
pub struct Gen {
    next: Var,
}

impl Gen {
    #[inline]
    pub const fn new() -> Self {
        Gen { next: Var::MIN }
    }

    pub fn new_sigs(&mut self, size: u32) -> Range<Var> {
        let start = self.next;
        self.next += size;
        let end = self.next;
        start..end
    }

    #[inline]
    pub fn next(&self) -> Var {
        self.next
    }
}

#[derive(Clone, Debug)]
pub enum Constraint {
    /// x + y = z
    /// CNF | (¬x ∨ ¬y) ∧ (¬x ∨ z) ∧ (x ∨ y ∨ ¬z) ∧ (¬y ∨ z)
    Linear { x: Var, y: Var, z: Var },
    /// assert [sign]x
    Assume { x: Var, sign: bool },
    /// x = y
    Equal { x: Var, y: Var },
    /// x <= y
    LessEqual { x: Var, y: Var },
    /// x = y and z
    EqMin { x: Var, y: Var, z: Var },
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Constraint::Linear { x, y, z } => f.write_fmt(format_args!("{x} + {y} = {z}")),
            Constraint::Assume { x, sign } => sign
                .then(|| f.write_fmt(format_args!("{x} = 1")))
                .unwrap_or_else(|| f.write_fmt(format_args!("{x} = 0"))),
            Constraint::Equal { x, y } => f.write_fmt(format_args!("{x} = {y}")),
            Constraint::LessEqual { x, y } => f.write_fmt(format_args!("{x} <= {y}")),
            Constraint::EqMin { x, y, z } => f.write_fmt(format_args!("{x} = min({y}, {z})")),
        }
    }
}

pub trait Mode {
    /// `Store<'a>` is a reference to a storage that holds
    /// the newly added constraint.
    type Store<'a>
    where
        Self: 'a;
    fn store_linear(store: Self::Store<'_>, x: Var, y: Var, z: Var);

    fn store_assumption(store: Self::Store<'_>, x: Var, sign: bool);

    fn store_equal(store: Self::Store<'_>, x: Var, y: Var);

    fn store_less_equal(store: Self::Store<'_>, x: Var, y: Var);

    fn store_eq_min(store: Self::Store<'_>, x: Var, y: Var, z: Var);
}

pub struct Emit;
impl Mode for Emit {
    type Store<'a> = &'a mut Vec<Constraint>;

    #[inline]
    fn store_linear(store: Self::Store<'_>, x: Var, y: Var, z: Var) {
        store.push(Constraint::Linear { x, y, z })
    }

    #[inline]
    fn store_assumption(store: Self::Store<'_>, x: Var, sign: bool) {
        store.push(Constraint::Assume { x, sign })
    }

    fn store_equal(store: Self::Store<'_>, x: Var, y: Var) {
        store.push(Constraint::Equal { x, y })
    }

    fn store_less_equal(store: Self::Store<'_>, x: Var, y: Var) {
        store.push(Constraint::LessEqual { x, y })
    }

    fn store_eq_min(store: Self::Store<'_>, x: Var, y: Var, z: Var) {
        store.push(Constraint::EqMin { x, y, z })
    }
}

macro_rules! tracing_for {
    (Debug, $args:tt) => {
        tracing::debug!($args)
    };
    (Info, $args:tt) => {
        tracing::info!($args)
    };
    (Warn, $args:tt) => {
        tracing::warn!($args)
    };
    (Error, $args:tt) => {
        tracing::error!($args)
    };
    (Print, $args:tt) => {
        println!($args)
    };
}

macro_rules! make_logging_mode {
    ($level:ident) => {
        pub struct $level;
        impl Mode for $level {
            type Store<'a> = ();

            #[inline]
            fn store_linear((): Self::Store<'_>, x: Var, y: Var, z: Var) {
                let constraint = Constraint::Linear { x, y, z };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            #[inline]
            fn store_assumption((): Self::Store<'_>, x: Var, sign: bool) {
                let constraint = Constraint::Assume { x, sign };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            fn store_equal((): Self::Store<'_>, x: Var, y: Var) {
                let constraint = Constraint::Equal { x, y };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            fn store_less_equal((): Self::Store<'_>, x: Var, y: Var) {
                let constraint = Constraint::LessEqual { x, y };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            fn store_eq_min((): Self::Store<'_>, x: Var, y: Var, z: Var) {
                let constraint = Constraint::EqMin { x, y, z };
                tracing_for!($level, "emitting constraint: {constraint}")
            }
        }
    };
}

make_logging_mode!(Debug);
make_logging_mode!(Info);
make_logging_mode!(Warn);
make_logging_mode!(Error);
make_logging_mode!(Print);

pub trait Database {
    #[inline]
    fn new_vars(&mut self, gen: &mut Gen, size: u32) -> Range<Var> {
        gen.new_sigs(size as u32)
    }

    fn push_linear_impl(&mut self, x: Var, y: Var, z: Var);
    fn push_linear<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, y: Var, z: Var) {
        self.push_linear_impl(x, y, z);
        Infer::store_linear(store, x, y, z);
    }
    fn push_assume_impl(&mut self, x: Var, sign: bool);
    fn push_assume<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, sign: bool) {
        self.push_assume_impl(x, sign);
        Infer::store_assumption(store, x, sign);
    }
    fn push_equal_impl(&mut self, x: Var, y: Var);
    fn push_equal<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, y: Var) {
        self.push_equal_impl(x, y);
        Infer::store_equal(store, x, y);
    }
    fn push_less_equal_impl(&mut self, x: Var, y: Var);
    fn push_less_equal<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, y: Var) {
        self.push_less_equal_impl(x, y);
        Infer::store_less_equal(store, x, y);
    }
    fn push_approx_linear_impl(&mut self, x: Var, y: Var, z: Var);
    fn push_approx_linear<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, y: Var, z: Var) {
        self.push_approx_linear_impl(x, y, z);
        Infer::store_linear(store, x, y, z);
    }

    fn push_eq_min_impl(&mut self, x: Var, y: Var, z: Var);
    fn push_eq_min<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: Var, y: Var, z: Var) {
        self.push_eq_min_impl(x, y, z);
        Infer::store_eq_min(store, x, y, z);
    }
}

impl Database for () {
    fn push_linear_impl(&mut self, _: Var, _: Var, _: Var) {}

    fn push_assume_impl(&mut self, _: Var, _: bool) {}

    fn push_equal_impl(&mut self, _: Var, _: Var) {}

    fn push_less_equal_impl(&mut self, _: Var, _: Var) {}

    fn push_approx_linear_impl(&mut self, _: Var, _: Var, _: Var) {}

    fn push_eq_min_impl(&mut self, _: Var, _: Var, _: Var) {}
}

pub struct CadicalDatabase {
    pub solver: cadical::Solver,
}

impl CadicalDatabase {
    pub fn new() -> Self {
        CadicalDatabase {
            solver: cadical::Solver::new(),
        }
    }
}

impl Database for CadicalDatabase {
    #[inline]
    fn push_linear_impl(&mut self, x: Var, y: Var, z: Var) {
        self.solver
            .add_clause([-x.into_lit(), -y.into_lit()].into_iter());
        self.solver
            .add_clause([-x.into_lit(), z.into_lit()].into_iter());
        self.solver
            .add_clause([x.into_lit(), y.into_lit(), -z.into_lit()].into_iter());
        self.solver
            .add_clause([-y.into_lit(), z.into_lit()].into_iter());
    }

    #[inline]
    fn push_assume_impl(&mut self, x: Var, sign: bool) {
        let mut lit = x.into_lit();
        if !sign {
            lit = -lit
        };
        self.solver.add_clause(std::iter::once(lit));
    }

    #[inline]
    fn push_equal_impl(&mut self, x: Var, y: Var) {
        // self.solver
        //     .add_clause([-x.into_lit(), y.into_lit()].into_iter());
        // self.solver
        //     .add_clause([x.into_lit(), -y.into_lit()].into_iter());
        self.push_less_equal_impl(x, y);
        self.push_less_equal_impl(y, x)
    }

    #[inline]
    fn push_less_equal_impl(&mut self, x: Var, y: Var) {
        self.solver
            .add_clause([-x.into_lit(), y.into_lit()].into_iter());
    }

    fn push_approx_linear_impl(&mut self, x: Var, y: Var, z: Var) {
        self.solver
            .add_clause([-x.into_lit(), -y.into_lit()].into_iter());
        self.solver
            .add_clause([-x.into_lit(), z.into_lit()].into_iter());
        self.solver
            .add_clause([-y.into_lit(), z.into_lit()].into_iter());
    }

    fn push_eq_min_impl(&mut self, x: Var, y: Var, z: Var) {
        self.solver
            .add_clause([-x.into_lit(), y.into_lit()].into_iter());
        self.solver
            .add_clause([-x.into_lit(), z.into_lit()].into_iter());
        self.solver
            .add_clause([x.into_lit(), -y.into_lit(), -z.into_lit()].into_iter());
    }
}

pub struct Z3Database<'z3> {
    pub ctx: &'z3 z3::Context,
    pub solver: z3::Solver<'z3>,
    pub z3_ast: IndexVec<Var, z3::ast::Bool<'z3>>,
}

impl<'z3> Z3Database<'z3> {
    pub fn new(ctx: &'z3 z3::Context) -> Self {
        let mut z3_ast = IndexVec::with_capacity(100);
        z3_ast.push(z3::ast::Bool::new_const(ctx, "dummy"));
        Z3Database {
            ctx,
            solver: z3::Solver::new(ctx),
            z3_ast,
        }
    }
}

impl<'z3> Database for Z3Database<'z3> {
    fn new_vars(&mut self, gen: &mut Gen, size: u32) -> Range<Var> {
        let sigs = gen.new_sigs(size);
        for sig in sigs.clone() {
            assert_eq!(
                sig,
                self.z3_ast
                    .push(z3::ast::Bool::new_const(self.ctx, sig.as_u32()))
            )
        }
        sigs
    }

    fn push_linear_impl(&mut self, x: Var, y: Var, z: Var) {
        let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[&!x, &!y]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, z]));
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[x, y, &!z]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!y, z]));
    }

    fn push_assume_impl(&mut self, x: Var, sign: bool) {
        let x = &self.z3_ast[x];
        let value = z3::ast::Bool::from_bool(self.ctx, sign);
        self.solver.assert(&!(x.xor(&value)))
        // self.solver.assert(&!z3::ast::Bool::xor(x, &value));
    }

    fn push_equal_impl(&mut self, x: Var, y: Var) {
        let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
        self.solver.assert(&!(x.xor(y)));
    }

    fn push_less_equal_impl(&mut self, x: Var, y: Var) {
        let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, y]));
    }

    fn push_approx_linear_impl(&mut self, x: Var, y: Var, z: Var) {
        let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[&!x, &!y]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, z]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!y, z]));
    }

    fn push_eq_min_impl(&mut self, x: Var, y: Var, z: Var) {
        let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, &y]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, z]));
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[x, &!y, &!z]));
    }
}
