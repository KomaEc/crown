use crate::UnitVar;

pub mod constraint_generation;
pub mod def_use;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    /// O(p) = O(q)
    Equality(Ownership, Ownership),
    /// O(q) = O(p') + O(q')
    Linear(Ownership, Ownership, Ownership),
}

rustc_index::newtype_index! {
    pub struct ConstraintIdx {
        DEBUG_FORMAT = "cstr_({})"
    }
}

rustc_index::newtype_index! {
    pub struct Ownership {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl UnitVar for Ownership {
    const ZERO: Self = Ownership::from_u32(0);
    const ONE: Self = Ownership::from_u32(1);
}
