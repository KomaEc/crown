#![feature(rustc_private)]
#![feature(step_trait)]

use std::{iter::Step, ops::Range};

use rustc_index::vec::Idx;

extern crate rustc_index;

/// This is a marker trait to mark index data types implemented
/// by rustc_index!
pub trait IsRustcIndex {}

pub trait RangeExt<T: IsRustcIndex + Idx + Step> {
    fn len(&self) -> usize;

    fn empty() -> Self;
}

impl<T> RangeExt<T> for Range<T>
where
    T: IsRustcIndex + Idx + Step,
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
