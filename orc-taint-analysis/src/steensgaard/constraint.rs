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

impl Constraint {
    #[inline]
    pub fn new(kind: ConstraintKind, p: AbstractLocation, q: AbstractLocation) -> Self {
        Constraint(kind, p, q)
    }
}

// pub struct ConstraintSet<'me>(&'me mut Vec<Constraint>);

// impl<'me> std::ops::Deref for ConstraintSet<'me> {
//     type Target = Vec<Constraint>;

//     fn deref(&self) -> &Self::Target {
//         &*self.0
//     }
// }

// impl<'me> std::ops::DerefMut for ConstraintSet<'me> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut *self.0
//     }
// }
