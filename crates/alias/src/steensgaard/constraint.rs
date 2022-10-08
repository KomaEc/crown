use super::AbstractLocation;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintKind {
    Addr,
    Assign,
    Store,
    Load,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Constraint(
    pub(super) ConstraintKind,
    pub(super) AbstractLocation,
    pub(super) AbstractLocation,
);

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ConstraintKind::Addr => write!(f, "{:?} = &{:?}", self.1, self.2),
            ConstraintKind::Assign => write!(f, "{:?} = {:?}", self.1, self.2),
            ConstraintKind::Store => write!(f, "*{:?} = &{:?}", self.1, self.2),
            ConstraintKind::Load => write!(f, "{:?} = *{:?}", self.1, self.2),
        }
    }
}

impl Constraint {
    #[inline]
    pub fn new(kind: ConstraintKind, p: AbstractLocation, q: AbstractLocation) -> Self {
        let constr = Constraint(kind, p, q);
        tracing::debug!("{constr}");
        constr
    }
}
