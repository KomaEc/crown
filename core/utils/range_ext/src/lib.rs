#![feature(rustc_private)]
#![feature(step_trait)]
#![feature(trait_alias)]

use std::{iter::Step, ops::Range};

use rustc_index::vec::Idx;

extern crate rustc_index;

/// This is a marker trait to mark constraint variable data types implemented
/// by rustc_index!
pub trait IsRustcIndexDefinedCV = IsConstraintVariable
    + Idx
    + Step
    + std::ops::Add<usize, Output = Self>
    + std::convert::From<usize>;

/// Marker trait that marks data types for constraint variables
pub trait IsConstraintVariable {}

pub trait RangeExt<T: IsConstraintVariable + Idx + Step> {
    fn len(&self) -> usize;

    fn empty() -> Self;

    fn split_first(&self) -> Option<(T, Range<T>)>;
}

impl<T> RangeExt<T> for Range<T>
where
    T: IsRustcIndexDefinedCV,
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

    fn split_first(&self) -> Option<(T, Range<T>)> {
        (!self.is_empty()).then(|| {
            (
                self.start,
                Range {
                    start: self.start + 1,
                    end: self.end,
                },
            )
        })
    }
}
