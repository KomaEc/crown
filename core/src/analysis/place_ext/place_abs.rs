use rustc_middle::mir::Local;


crate::macros::newtype_index! {
    pub struct FieldOffset {
        DEBUG_FORMAT = ".({})"
    }
}

/// place abstraction ignores all projections except derefs
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlaceAbs {
    pub base_local: Local,
    pub num_derefs: u32,
}

impl PlaceAbs {
    #[inline]
    fn deref(mut self) -> Self {
        self.num_derefs += 1;
        self
    }
}
