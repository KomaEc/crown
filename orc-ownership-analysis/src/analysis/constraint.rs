use std::ops::Range;

use rustc_middle::mir::LocalDecl;

use crate::{
    analysis::{def::maybe_owned, ty::ty_ptr_measure},
    CrateCtxt,
};

pub mod infer;

orc_common::macros::orc_index!(OwnershipSig);

impl std::fmt::Display for OwnershipSig {
    // \mathbb{O}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("𝕆({:?})", *self))
    }
}

impl OwnershipSig {
    // pub const MIN: Self = OwnershipSig::from_u32(1);

    pub fn into_lit(self) -> i32 {
        self.as_u32() as i32
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

pub struct OwnershipSigGenerator {
    next: OwnershipSig,
}

impl OwnershipSigGenerator {
    pub fn new(start: OwnershipSig) -> Self {
        OwnershipSigGenerator { next: start }
    }

    pub fn gen(&mut self, num: u32) -> Range<OwnershipSig> {
        let start = self.next;
        self.next += num;
        let end = self.next;
        start..end
    }

    #[inline]
    pub fn next(&self) -> OwnershipSig {
        self.next
    }
}

#[inline]
pub fn generate_signatures_for_local<'tcx>(
    local_decl: &LocalDecl<'tcx>,
    gen: &mut OwnershipSigGenerator,
    crate_ctxt: &CrateCtxt<'tcx>,
) -> Option<Range<OwnershipSig>> {
    maybe_owned(local_decl, crate_ctxt).then(|| {
        let ty = local_decl.ty;
        let measure = ty_ptr_measure(ty, crate_ctxt);
        gen.gen(measure)
    })
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
    const FIRST_AVAILABLE_SIG: OwnershipSig;

    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig);
    fn push_linear<M: Mode>(
        &mut self,
        store: M::Store<'_>,
        x: OwnershipSig,
        y: OwnershipSig,
        z: OwnershipSig,
    ) {
        self.push_linear_impl(x, y, z);
        M::store_linear(store, x, y, z);
    }
    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool);
    fn push_assume<M: Mode>(&mut self, store: M::Store<'_>, x: OwnershipSig, sign: bool) {
        self.push_assume_impl(x, sign);
        M::store_assumption(store, x, sign);
    }
    fn push_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig);
    fn push_equal<M: Mode>(&mut self, store: M::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
        self.push_equal_impl(x, y);
        M::store_equal(store, x, y);
    }
    fn push_less_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig);
    fn push_less_equal<M: Mode>(&mut self, store: M::Store<'_>, x: OwnershipSig, y: OwnershipSig) {
        self.push_less_equal_impl(x, y);
        M::store_less_equal(store, x, y);
    }
}

impl Database for () {
    const FIRST_AVAILABLE_SIG: OwnershipSig = OwnershipSig::from_u32(0);

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
    const FIRST_AVAILABLE_SIG: OwnershipSig = OwnershipSig::from_u32(1);

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


impl<'z3> Database for z3::Solver<'z3> {
    const FIRST_AVAILABLE_SIG: OwnershipSig = OwnershipSig::from_u32(1);

    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {
        todo!()
    }

    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool) {
        todo!()
    }

    fn push_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        todo!()
    }

    fn push_less_equal_impl(&mut self, x: OwnershipSig, y: OwnershipSig) {
        todo!()
    }
}
