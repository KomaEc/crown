pub(crate) mod infer;

orc_common::macros::orc_index!(OwnershipSig);

impl std::fmt::Display for OwnershipSig {
    // \mathbb{O}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("𝕆({:?})", *self))
    }
}

impl OwnershipSig {
    pub(crate) const MIN: Self = OwnershipSig::from_u32(1);

    pub(crate) fn into_lit(self) -> i32 {
        self.as_u32() as i32
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Constraint {
    /// x + y = z
    /// CNF | (¬x ∨ ¬y) ∧ (¬x ∨ z) ∧ (x ∨ y ∨ ¬z) ∧ (¬y ∨ z)
    Linear {
        x: OwnershipSig,
        y: OwnershipSig,
        z: OwnershipSig,
    },
    /// assert [sign]x
    Assume { x: OwnershipSig, sign: bool },
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Constraint::Linear { x, y, z } => f.write_fmt(format_args!("{x} + {y} = {z}")),
            Constraint::Assume { x, sign } => sign
                .then(|| f.write_fmt(format_args!("{x} = 1")))
                .unwrap_or_else(|| f.write_fmt(format_args!("{x} = 0"))),
        }
    }
}

pub(crate) trait Mode {
    /// `Store<'a>` is a reference to a storage that holds
    /// the newly added constraint.
    type Store<'a>
    where
        Self: 'a;
    fn store_linear(store: Self::Store<'_>, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig);

    fn store_assumption(store: Self::Store<'_>, x: OwnershipSig, sign: bool);
}

pub(crate) struct Emit;
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
        pub(crate) struct $level;
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
        }
    };
}

make_logging_mode!(Debug);
make_logging_mode!(Info);
make_logging_mode!(Warn);
make_logging_mode!(Error);

pub(crate) trait Database {
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
}

impl Database for () {
    fn push_linear_impl(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {}

    fn push_assume_impl(&mut self, x: OwnershipSig, sign: bool) {}
}

pub(crate) struct CadicalDatabase {
    pub(crate) solver: cadical::Solver,
}

impl CadicalDatabase {
    pub(crate) fn new() -> Self {
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
}
