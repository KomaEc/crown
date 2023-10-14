use std::ops::Range;

use rustc_index::IndexVec;

utils::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct OwnershipToken {
    }
}

impl std::fmt::Display for OwnershipToken {
    // \mathbb{O}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("𝕆({})", self.as_u32()))
    }
}

impl OwnershipToken {
    pub const INVALID: Self = OwnershipToken::from_u32(0);
    pub const MIN: Self = OwnershipToken::from_u32(1);

    pub fn into_lit(self) -> i32 {
        self.as_u32() as i32
    }
}

impl std::ops::Add<u32> for OwnershipToken {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        OwnershipToken::from_u32(self.as_u32() + rhs)
    }
}

impl std::ops::AddAssign<u32> for OwnershipToken {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs
    }
}

pub struct OwnershipTokenGenerator {
    next: OwnershipToken,
}

impl OwnershipTokenGenerator {
    #[inline]
    pub const fn new() -> Self {
        OwnershipTokenGenerator {
            next: OwnershipToken::MIN,
        }
    }

    pub fn new_tokens(&mut self, size: usize) -> Range<OwnershipToken> {
        let start = self.next;
        self.next += size as u32;
        let end = self.next;
        start..end
    }

    #[inline]
    pub fn next(&self) -> OwnershipToken {
        self.next
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Constraint {
    /// x + y = z
    /// CNF | (¬x ∨ ¬y) ∧ (¬x ∨ z) ∧ (x ∨ y ∨ ¬z) ∧ (¬y ∨ z)
    Linear {
        x: OwnershipToken,
        y: OwnershipToken,
        z: OwnershipToken,
    },
    /// assert [sign]x
    Assume { x: OwnershipToken, sign: bool },
    /// x = y
    Equal {
        x: OwnershipToken,
        y: OwnershipToken,
    },
    /// x <= y
    LessEqual {
        x: OwnershipToken,
        y: OwnershipToken,
    },
    /// min(x, y) = z
    Min {
        x: OwnershipToken,
        y: OwnershipToken,
        z: OwnershipToken,
    },
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
            Constraint::Min { x, y, z } => f.write_fmt(format_args!("min({x}, {y}) = {z}")),
        }
    }
}

pub trait StorageMode {
    type Storage;
    fn store(storage: &mut Self::Storage, constraint: Constraint);
}

pub enum Emit {}
impl StorageMode for Emit {
    type Storage = Vec<Constraint>;

    fn store(storage: &mut Self::Storage, constraint: Constraint) {
        storage.push(constraint)
    }
}

macro_rules! tracing_msg {
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
        impl StorageMode for $level {
            type Storage = ();
            fn store((): &mut (), constraint: Constraint) {
                tracing_msg!($level, "emitting constraint: {constraint}")
            }
        }
    };
}

make_logging_mode!(Debug);
make_logging_mode!(Info);
make_logging_mode!(Warn);
make_logging_mode!(Error);
make_logging_mode!(Print);

impl<T, U> StorageMode for (T, U)
where
    T: StorageMode,
    U: StorageMode,
{
    type Storage = (T::Storage, U::Storage);

    fn store(storage: &mut Self::Storage, constraint: Constraint) {
        T::store(&mut storage.0, constraint);
        U::store(&mut storage.1, constraint)
    }
}

pub trait Database<Mode: StorageMode = Debug> {
    fn new_tokens(&mut self, size: usize) -> Range<OwnershipToken>;

    fn add(&mut self, constraint: Constraint, storage: &mut Mode::Storage) {
        self.add_inner(constraint);
        Mode::store(storage, constraint);
    }
    fn add_inner(&mut self, constraint: Constraint);
}

pub struct CadicalDatabase {
    pub solver: cadical::Solver,
    pub gen: OwnershipTokenGenerator,
}

impl CadicalDatabase {
    pub fn new() -> Self {
        CadicalDatabase {
            solver: cadical::Solver::new(),
            gen: OwnershipTokenGenerator::new(),
        }
    }

    /// add `x <= y`
    fn add_le(&mut self, x: OwnershipToken, y: OwnershipToken) {
        self.solver
            .add_clause([-x.into_lit(), y.into_lit()].into_iter())
    }
}

impl<Mode: StorageMode> Database<Mode> for CadicalDatabase {
    fn new_tokens(&mut self, size: usize) -> Range<OwnershipToken> {
        self.gen.new_tokens(size)
    }

    fn add_inner(&mut self, constraint: Constraint) {
        match constraint {
            Constraint::Linear { x, y, z } => {
                self.solver
                    .add_clause([-x.into_lit(), -y.into_lit()].into_iter());
                self.solver
                    .add_clause([-x.into_lit(), z.into_lit()].into_iter());
                self.solver
                    .add_clause([x.into_lit(), y.into_lit(), -z.into_lit()].into_iter());
                self.solver
                    .add_clause([-y.into_lit(), z.into_lit()].into_iter());
            }
            Constraint::Assume { x, sign } => {
                let mut lit = x.into_lit();
                if !sign {
                    lit = -lit
                };
                self.solver.add_clause(std::iter::once(lit));
            }
            Constraint::Equal { x, y } => {
                self.add_le(x, y);
                self.add_le(y, x);
            }
            Constraint::LessEqual { x, y } => self.add_le(x, y),
            Constraint::Min { x, y, z } => {
                self.add_le(z, x);
                self.add_le(z, y);
                self.solver
                    .add_clause([-x.into_lit(), -y.into_lit(), z.into_lit()].into_iter());
            }
        }
    }
}

pub struct Z3Database<'z3> {
    pub ctx: &'z3 z3::Context,
    pub solver: z3::Solver<'z3>,
    pub z3_ast: IndexVec<OwnershipToken, z3::ast::Bool<'z3>>,
    pub gen: OwnershipTokenGenerator,
}

impl<'z3> Z3Database<'z3> {
    pub fn new(ctx: &'z3 z3::Context) -> Self {
        let mut z3_ast = IndexVec::with_capacity(100);
        z3_ast.push(z3::ast::Bool::new_const(ctx, "dummy"));
        Z3Database {
            ctx,
            solver: z3::Solver::new(ctx),
            z3_ast,
            gen: OwnershipTokenGenerator::new(),
        }
    }
}

impl<'z3, Mode: StorageMode> Database<Mode> for Z3Database<'z3> {
    fn new_tokens(&mut self, size: usize) -> Range<OwnershipToken> {
        let tokens = self.gen.new_tokens(size);
        for token in tokens.clone() {
            assert_eq!(
                token,
                self.z3_ast
                    .push(z3::ast::Bool::new_const(self.ctx, token.as_u32()))
            )
        }

        tokens
    }

    fn add_inner(&mut self, constraint: Constraint) {
        match constraint {
            Constraint::Linear { x, y, z } => {
                let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
                self.solver
                    .assert(&z3::ast::Bool::or(self.ctx, &[&!x, &!y]));
                self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, z]));
                self.solver
                    .assert(&z3::ast::Bool::or(self.ctx, &[x, y, &!z]));
                self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!y, z]));
            }
            Constraint::Assume { x, sign } => {
                let x = &self.z3_ast[x];
                let value = z3::ast::Bool::from_bool(self.ctx, sign);
                self.solver.assert(&!(x.xor(&value)))
            }
            Constraint::Equal { x, y } => {
                let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
                self.solver.assert(&!(x.xor(y)));
            }
            Constraint::LessEqual { x, y } => {
                let [x, y] = [x, y].map(|sig| &self.z3_ast[sig]);
                self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!x, y]));
            }
            Constraint::Min { x, y, z } => {
                let [x, y, z] = [x, y, z].map(|sig| &self.z3_ast[sig]);
                self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!z, x]));
                self.solver.assert(&z3::ast::Bool::or(self.ctx, &[&!z, y]));
                self.solver
                    .assert(&z3::ast::Bool::or(self.ctx, &[&!x, &!y, z]));
            }
        }
    }
}
