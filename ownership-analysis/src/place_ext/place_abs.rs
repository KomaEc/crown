use rustc_middle::mir::Local;

crate::macros::newtype_index! {
    pub struct AggregateOffset {
        DEBUG_FORMAT = ".{}"
    }
}

impl std::ops::Add<AggregateOffset> for AggregateOffset {
    type Output = Self;

    fn add(self, rhs: AggregateOffset) -> Self::Output {
        self + rhs.as_usize()
    }
}

impl std::ops::AddAssign for AggregateOffset {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

/// TODO: place abs is useful only for aggregated data
/// type. If _1 is an i32, then there is no need to abstract _1.
/// place abstraction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlaceAbs {
    pub base_local: Local,
    pub dereferenced: bool,
    pub offset: AggregateOffset,
}

impl std::fmt::Display for PlaceAbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dereferenced {
            f.write_str("(*")?
        }
        f.write_fmt(format_args!("{:?}", self.base_local))?;
        if self.dereferenced {
            f.write_str(")")?
        }
        f.write_fmt(format_args!("{:?}", self.offset))
    }
}

impl PlaceAbs {
    pub fn from_local(local: Local) -> Self {
        PlaceAbs {
            base_local: local,
            dereferenced: false,
            offset: 0u32.into(),
        }
    }
}
