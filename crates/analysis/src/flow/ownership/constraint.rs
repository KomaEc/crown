use std::ops::Range;

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

pub struct OwnershipTokenGen {
    next: OwnershipToken,
}

impl OwnershipTokenGen {
    #[inline]
    pub const fn new() -> Self {
        OwnershipTokenGen {
            next: OwnershipToken::MIN,
        }
    }

    pub fn new_sigs(&mut self, size: u32) -> Range<OwnershipToken> {
        let start = self.next;
        self.next += size;
        let end = self.next;
        start..end
    }

    #[inline]
    pub fn next(&self) -> OwnershipToken {
        self.next
    }
}

#[derive(Clone, Debug)]
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
    /// x = min(y, z)
    EqMin {
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
            Constraint::EqMin { x, y, z } => f.write_fmt(format_args!("{x} = min({y}, {z})")),
        }
    }
}
