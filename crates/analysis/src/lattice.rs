
pub trait Lattice: Copy + Eq + HasBottom + HasTop {
    fn join(self, other: Self) -> Self;
    fn meet(self, other: Self) -> Self;
}

pub trait HasBottom {
    const BOTTOM: Self;
}

pub trait HasTop {
    const TOP: Self;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FlatSet<T> {
    Bottom,
    Elem(T),
    Top,
}

impl<T> FlatSet<T> {
    pub fn dual(self) -> Self {
        match self {
            FlatSet::Bottom => FlatSet::Top,
            FlatSet::Elem(_) => self,
            FlatSet::Top => FlatSet::Bottom,
        }
    }
}

impl<T> HasBottom for FlatSet<T> {
    const BOTTOM: Self = FlatSet::Bottom;
}

impl<T> HasTop for FlatSet<T> {
    const TOP: Self = FlatSet::Top;
}

impl<T> Lattice for FlatSet<T> where T: Clone + Copy + Eq {
    fn join(self, other: Self) -> Self {
        match (self, other) {
            (FlatSet::Top, _) | (_, FlatSet::Top) => FlatSet::Top,
            (FlatSet::Bottom, elem) | (elem, FlatSet::Bottom) => elem,
            (FlatSet::Elem(this), FlatSet::Elem(that)) if this == that => self,
            _ => FlatSet::Top
        }
    }

    fn meet(self, other: Self) -> Self {
        Lattice::join(self, other).dual()
    }
}

impl HasBottom for bool {
    const BOTTOM: Self = false;
}

impl HasTop for bool {
    const TOP: Self = true;
}

impl Lattice for bool {
    fn join(self, other: Self) -> Self {
        self || other
    }

    fn meet(self, other: Self) -> Self {
        self && other
    }
}
