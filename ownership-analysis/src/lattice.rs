use rustc_index::vec::{Idx, IndexVec};

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

impl<I: Idx> JoinLattice for IndexVec<I, Option<bool>> {
    fn join(&mut self, other: &Self) -> Result<bool, ()> {
        assert_eq!(self.len(), other.len());
        let mut changed = false;
        for (this, that) in std::iter::zip(self.iter_mut(), other.iter()) {
            changed = this.join(that)?;
        }
        Ok(changed)
    }

    fn bottom() -> Self {
        panic!("the bottom value should be manually created!")
    }
}
