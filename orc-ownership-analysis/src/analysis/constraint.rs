use std::ops::Range;

use rustc_index::vec::IndexVec;
use rustc_middle::mir::LocalDecl;

use crate::{analysis::consume::try_measure_local, ptr::Measurable};

use super::consume::HasInvalid;

pub mod infer;
// pub mod prune;

orc_common::macros::orc_index!(OwnershipSig);

impl std::fmt::Display for OwnershipSig {
    // \mathbb{O}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("𝕆({})", self.as_u32()))
    }
}

impl OwnershipSig {
    pub const INVALID: Self = OwnershipSig::from_u32(0);
    pub const MIN: Self = OwnershipSig::from_u32(1);

    pub fn into_lit(self) -> i32 {
        self.as_u32() as i32
    }
}

impl HasInvalid for Range<OwnershipSig> {
    const INVALID: Self = Range {
        start: OwnershipSig::INVALID,
        end: OwnershipSig::INVALID,
    };

    #[inline]
    fn is_invalid(&self) -> bool {
        self.start == OwnershipSig::INVALID && self.end == OwnershipSig::INVALID
    }
}

impl std::ops::Add<u32> for OwnershipSig {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        OwnershipSig::from_u32(self.as_u32() + rhs)
    }
}

impl std::ops::AddAssign<u32> for OwnershipSig {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs
    }
}

pub struct Gen {
    next: OwnershipSig,
}

impl Gen {
    #[inline]
    pub const fn new() -> Self {
        Gen {
            next: OwnershipSig::MIN,
        }
    }

    pub fn new_sigs(&mut self, size: u32) -> Range<OwnershipSig> {
        let start = self.next;
        self.next += size;
        let end = self.next;
        start..end
    }

    #[inline]
    pub fn next(&self) -> OwnershipSig {
        self.next
    }
}

#[inline]
pub fn generate_signatures_for_local(
    local_decl: &LocalDecl,
    gen: &mut Gen,
    database: &mut impl Database,
    measurable: impl Measurable,
) -> Option<Range<OwnershipSig>> {
    try_measure_local(local_decl, measurable)
        .map(|measure| database.new_vars(gen.new_sigs(measure.get())))
}

#[derive(Clone, Debug)]
pub enum Constraint {
    /// x + y = z
    /// CNF | (¬x ∨ ¬y) ∧ (¬x ∨ z) ∧ (x ∨ y ∨ ¬z) ∧ (¬y ∨ z)
    Linear {
        x: OwnershipSig,
        y: OwnershipSig,
        z: OwnershipSig,
    },
    /// assert [sign]x
    Assume { x: OwnershipSig, sign: bool },
    /// x = y
    Equal { x: OwnershipSig, y: OwnershipSig },
    /// x <= y
    LessEqual { x: OwnershipSig, y: OwnershipSig },
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
        }
    }
}

pub trait Mode {
    /// `Store<'a>` is a reference to a storage that holds
    /// the newly added constraint.
    type Store<'a>
    where
        Self: 'a;
    fn store_linear(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig);

    fn store_assumption(store: Self::Store<'_>, x: OwnershipSig, sign: bool);

    fn store_equal(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig);

    fn store_less_equal(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig);
}

pub struct Emit;
impl Mode for Emit {
    type Store<'a> = &'a mut Vec<Constraint>;

    #[inline]
    fn store_linear(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {
        store.push(Constraint::Linear { x, y, z })
    }

    #[inline]
    fn store_assumption(store: Self::Store<'_>, x: OwnershipSig, sign: bool) {
        store.push(Constraint::Assume { x, sign })
    }

    fn store_equal(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
        store.push(Constraint::Equal { x, y })
    }

    fn store_less_equal(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
        store.push(Constraint::LessEqual { x, y })
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
}

macro_rules! make_logging_mode {
    ($level:ident) => {
        pub struct $level;
        impl Mode for $level {
            type Store<'a> = ();

            #[inline]
            fn store_linear(
                (): Self::Store<'_>,
                x: OwnershipSig,
                y: OwnershipSig,
                z: OwnershipSig,
            ) {
                let constraint = Constraint::Linear { x, y, z };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            #[inline]
            fn store_assumption((): Self::Store<'_>, x: OwnershipSig, sign: bool) {
                let constraint = Constraint::Assume { x, sign };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            fn store_equal((): Self::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
                let constraint = Constraint::Equal { x, y };
                tracing_for!($level, "emitting constraint: {constraint}")
            }

            fn store_less_equal((): Self::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
                let constraint = Constraint::LessEqual { x, y };
                tracing_for!($level, "emitting constraint: {constraint}")
            }
        }
    };
}

make_logging_mode!(Debug);
make_logging_mode!(Info);
make_logging_mode!(Warn);
make_logging_mode!(Error);

pub trait Database {
    #[inline]
    fn new_var(&mut self, _: OwnershipSig) {}

    #[inline]
    fn new_vars(&mut self, sigs: Range<OwnershipSig>) -> Range<OwnershipSig> {
        sigs
    }

    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig);
    fn push_linear<Infer: Mode>(
        &mut self,
        store: Infer::Store<'_>,
        x: OwnershipSig,
        y: OwnershipSig,
        z: OwnershipSig,
    ) {
        self.push_linear_impl(x, y, z);
        Infer::store_linear(store, x, y, z);
    }
    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool);
    fn push_assume<Infer: Mode>(&mut self, store: Infer::Store<'_>, x: OwnershipSig, sign: bool) {
        self.push_assume_impl(x, sign);
        Infer::store_assumption(store, x, sign);
    }
    fn push_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig);
    fn push_equal<Infer: Mode>(
        &mut self,
        store: Infer::Store<'_>,
        x: OwnershipSig,
        y: OwnershipSig,
    ) {
        self.push_equal_impl(x, y);
        Infer::store_equal(store, x, y);
    }
    fn push_less_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig);
    fn push_less_equal<Infer: Mode>(
        &mut self,
        store: Infer::Store<'_>,
        x: OwnershipSig,
        y: OwnershipSig,
    ) {
        self.push_less_equal_impl(x, y);
        Infer::store_less_equal(store, x, y);
    }
}

impl Database for () {
    fn push_linear_impl(&mut self, _: OwnershipSig, _: OwnershipSig, _: OwnershipSig) {}

    fn push_assume_impl(&mut self, _: OwnershipSig, _: bool) {}

    fn push_equal_impl(&mut self, _: OwnershipSig, _: OwnershipSig) {}

    fn push_less_equal_impl(&mut self, _: OwnershipSig, _: OwnershipSig) {}
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
    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {
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
    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool) {
        let mut lit = x.into_lit();
        if !sign {
            lit = -lit
        };
        self.solver.add_clause(std::iter::once(lit));
    }

    #[inline]
    fn push_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        // self.solver
        //     .add_clause([-x.into_lit(), y.into_lit()].into_iter());
        // self.solver
        //     .add_clause([x.into_lit(), -y.into_lit()].into_iter());
        self.push_less_equal_impl(x, y);
        self.push_less_equal_impl(y, x)
    }

    #[inline]
    fn push_less_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        self.solver
            .add_clause([-x.into_lit(), y.into_lit()].into_iter());
    }
}

pub struct Z3Database<'z3> {
    pub ctx: &'z3 z3::Context,
    pub solver: z3::Solver<'z3>,
    pub z3_ast: IndexVec<OwnershipSig, z3::ast::Bool<'z3>>,
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
    fn new_var(&mut self, x: OwnershipSig) {
        assert_eq!(
            x,
            self.z3_ast
                .push(z3::ast::Bool::new_const(self.ctx, x.as_u32()))
        )
    }

    fn new_vars(&mut self, sigs: Range<OwnershipSig>) -> Range<OwnershipSig> {
        for sig in sigs.clone() {
            self.new_var(sig)
        }
        sigs
    }

    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {
        let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[&!x, &!y]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, z]));
        self.solver
            .assert(&z3::ast::Bool::or(self.ctx, &[x, y, &!z]));
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!y, z]));
    }

    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool) {
        let x = &self.z3_ast[x];
        let value = z3::ast::Bool::from_bool(self.ctx, sign);
        self.solver.assert(&!(x.xor(&value)))
        // self.solver.assert(&!z3::ast::Bool::xor(x, &value));
    }

    fn push_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
        self.solver.assert(&!(x.xor(y)));
    }

    fn push_less_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
        self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, y]));
    }
}
