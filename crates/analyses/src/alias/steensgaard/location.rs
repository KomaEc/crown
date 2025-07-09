use utils::petgraph;

pub enum PlaceLocation {
    Plain(AbstractLocation),
    Deref(AbstractLocation),
}

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "{}"]
    pub struct AbstractLocation {
    }
}

impl Default for AbstractLocation {
    fn default() -> Self {
        Self::from_u32(0)
    }
}

unsafe impl petgraph::graph::IndexType for AbstractLocation {
    fn new(x: usize) -> Self {
        AbstractLocation::from_usize(x)
    }

    fn index(&self) -> usize {
        <Self as rustc_index::Idx>::index(*self)
    }

    fn max() -> Self {
        Self::MAX
    }
}

impl AbstractLocation {
    pub const NULL: Self = AbstractLocation::from_u32(0);

    pub fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}

impl std::ops::Add<u32> for AbstractLocation {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        self + (rhs as usize)
    }
}

impl std::ops::AddAssign<u32> for AbstractLocation {
    #[inline]
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs
    }
}
