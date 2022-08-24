pub mod gathering;

orc_common::macros::orc_index!(OwnershipSig);

impl OwnershipSig {
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

pub trait Mode {
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
pub(crate) struct NoEmit;
impl Mode for NoEmit {
    type Store<'a> = ();

    #[inline]
    fn store_linear(_store: Self::Store<'_>, _x: OwnershipSig, _y: OwnershipSig, _z: OwnershipSig) {
    }

    #[inline]
    fn store_assumption(_store: Self::Store<'_>, _x: OwnershipSig, _sign: bool) {}
}

pub trait Database {
    fn add_linear(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig);
    fn push_linear<M: Mode>(
        &mut self,
        store: M::Store<'_>,
        x: OwnershipSig,
        y: OwnershipSig,
        z: OwnershipSig,
    ) {
        self.add_linear(x, y, z);
        M::store_linear(store, x, y, z);
    }
    fn add_assume(&mut self, x: OwnershipSig, sign: bool);
    fn push_assume<M: Mode>(&mut self, store: M::Store<'_>, x: OwnershipSig, sign: bool) {
        self.add_assume(x, sign);
        M::store_assumption(store, x, sign);
    }
}

pub struct CadicalDatabase {
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
    fn add_linear(&mut self, x: OwnershipSig, y: OwnershipSig, z: OwnershipSig) {
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
    fn add_assume(&mut self, x: OwnershipSig, sign: bool) {
        let mut lit = x.into_lit();
        if !sign {
            lit = -lit
        };
        self.solver.add_clause(std::iter::once(lit));
    }
}
