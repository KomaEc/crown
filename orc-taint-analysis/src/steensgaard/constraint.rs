use super::AbstractLocation;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintKind {
    Addr,
    Assign,
    Store,
    Load,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Constraint(pub(super) ConstraintKind, pub(super) AbstractLocation, pub(super) AbstractLocation);

impl Constraint {
    #[inline]
    pub fn new(kind: ConstraintKind, p: AbstractLocation, q: AbstractLocation) -> Self {
        Constraint(kind, p, q)
    }
}
