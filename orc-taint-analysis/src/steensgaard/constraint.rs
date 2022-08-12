use crate::AbstractLocation;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintKind {
    Addr,
    Assign,
    Store,
    Load,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Constraint(ConstraintKind, AbstractLocation, AbstractLocation);


