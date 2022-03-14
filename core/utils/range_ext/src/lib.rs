#![feature(rustc_private)]
#![feature(step_trait)]

use std::{iter::Step, ops::Range};

use rustc_index::vec::Idx;

extern crate rustc_index;

/// This is a marker trait to mark constraint variable data types implemented
/// by rustc_index!
pub trait IsConstraintVariable {}

pub trait RangeExt<T: IsConstraintVariable + Idx + Step> {
    fn len(&self) -> usize;

    fn empty() -> Self;
}

impl<T> RangeExt<T> for Range<T>
where
    T: IsConstraintVariable + Idx + Step,
{
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        // Note: This assertion is overly defensive, but it checks the invariant
        // guaranteed by the trait. If this trait were rust-internal,
        // we could use debug_assert!; assert_eq! will check all Rust user
        // implementations too.
        assert_eq!(upper, Some(lower));
        lower
    }

    fn empty() -> Self {
        let max = T::new(0xFFFF_FF00);
        Range {
            start: max,
            end: max,
        }
    }
}
