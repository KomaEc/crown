/// A join lattice with the semantic that the top value indicates
/// an error
pub trait JoinLattice {
    fn join(&mut self, other: &Self) -> Result<bool, ()>;
    fn bottom() -> Self;
}

impl JoinLattice for Option<bool> {
    fn join(&mut self, other: &Self) -> Result<bool, ()> {
        match (*self, *other) {
            (None, None) => Ok(false),
            (None, Some(_)) => {
                *self = *other;
                Ok(true)
            }
            (Some(_), None) => Ok(false),
            (Some(this), Some(that)) if this ^ that => Err(()),
            _ => Ok(false),
        }
    }

    fn bottom() -> Self {
        None
    }
}
