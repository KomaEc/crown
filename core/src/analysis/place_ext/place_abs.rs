use rustc_middle::mir::Local;

crate::macros::newtype_index! {
    pub struct AggregateOffset {
        DEBUG_FORMAT = ".({})"
    }
}

/// place abstraction
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaceAbs {
    pub base_local: Local,
    pub dereferenced: bool,
    pub offset: AggregateOffset,
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
