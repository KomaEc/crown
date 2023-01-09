//! Macros defined or re-exported from rustc

/// Implement `IndexType` trait of petgraph for an index type.
/// Note that `IndexType` should not be imported elsewhere.
#[macro_export]
macro_rules! petgraph_index {
    ($I:ident) => {
        static_assertions::assert_impl_all!($I: rustc_index::vec::Idx, Default);

        unsafe impl petgraph::graph::IndexType for $I {
            #[inline(always)]
            fn new(x: usize) -> Self {
                $I::from_usize(x)
            }

            #[inline(always)]
            fn index(&self) -> usize {
                <$I as rustc_index::vec::Idx>::index(*self)
            }

            #[inline(always)]
            fn max() -> Self {
                $I::MAX
            }
        }
    };
}

pub use petgraph_index;
#[cfg(not(debug_assertions))]
pub use rustc_index::newtype_index;

/// A hack that makes rust-analyzer work on vscode
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! newtype_index {
    // ---- public rules ----

// Use default constants
($(#[$attrs:meta])* $v:vis struct $name:ident {  }) => (
    $crate::newtype_index!(
        // Ignore everything
        @attrs        []
        @type         [$name]
        // shave off 256 indices at the end to allow space for packing these indices into enums
        @max          [0xFFFF_FF00]
        @vis          [$v]
        @debug_format ["{}"]);
);


    // ---- private rules ----

    // Base case, user-defined constants (if any) have already been defined
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]) => (
        $(#[$attrs])*
        #[derive(Copy, PartialEq, Eq, Hash, PartialOrd, Ord, $($derives),*)]
        // #[rustc_layout_scalar_valid_range_end($max)]
        $v struct $type {
            private: u32
        }

        impl Clone for $type {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        #[allow(dead_code)]
        impl $type {
            /// Maximum value the index can take, as a `u32`.
            $v const MAX_AS_U32: u32 = $max;

            /// Maximum value the index can take.
            $v const MAX: Self = Self::from_u32($max);

            /// Creates a new index from a given `usize`.
            ///
            /// # Panics
            ///
            /// Will panic if `value` exceeds `MAX`.
            #[inline]
            $v const fn from_usize(value: usize) -> Self {
                assert!(value <= ($max as usize));
                // SAFETY: We just checked that `value <= max`.
                unsafe {
                    Self::from_u32_unchecked(value as u32)
                }
            }

            /// Creates a new index from a given `u32`.
            ///
            /// # Panics
            ///
            /// Will panic if `value` exceeds `MAX`.
            #[inline]
            $v const fn from_u32(value: u32) -> Self {
                assert!(value <= $max);
                // SAFETY: We just checked that `value <= max`.
                unsafe {
                    Self::from_u32_unchecked(value)
                }
            }

            /// Creates a new index from a given `u32`.
            ///
            /// # Safety
            ///
            /// The provided value must be less than or equal to the maximum value for the newtype.
            /// Providing a value outside this range is undefined due to layout restrictions.
            ///
            /// Prefer using `from_u32`.
            #[inline]
            $v const unsafe fn from_u32_unchecked(value: u32) -> Self {
                Self { private: value }
            }

            /// Extracts the value of this index as a `usize`.
            #[inline]
            $v const fn index(self) -> usize {
                self.as_usize()
            }

            /// Extracts the value of this index as a `u32`.
            #[inline]
            $v const fn as_u32(self) -> u32 {
                self.private
            }

            /// Extracts the value of this index as a `usize`.
            #[inline]
            $v const fn as_usize(self) -> usize {
                self.as_u32() as usize
            }
        }

        impl std::ops::Add<usize> for $type {
            type Output = Self;

            fn add(self, other: usize) -> Self {
                Self::from_usize(self.index() + other)
            }
        }

        impl rustc_index::vec::Idx for $type {
            #[inline]
            fn new(value: usize) -> Self {
                Self::from_usize(value)
            }

            #[inline]
            fn index(self) -> usize {
                self.as_usize()
            }
        }

        impl ::std::iter::Step for $type {
            #[inline]
            fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                <usize as ::std::iter::Step>::steps_between(
                    &Self::index(*start),
                    &Self::index(*end),
                )
            }

            #[inline]
            fn forward_checked(start: Self, u: usize) -> Option<Self> {
                Self::index(start).checked_add(u).map(Self::from_usize)
            }

            #[inline]
            fn backward_checked(start: Self, u: usize) -> Option<Self> {
                Self::index(start).checked_sub(u).map(Self::from_usize)
            }
        }

        // Safety: The implementation of `Step` upholds all invariants.
        unsafe impl ::std::iter::TrustedStep for $type {}

        impl From<$type> for u32 {
            #[inline]
            fn from(v: $type) -> u32 {
                v.as_u32()
            }
        }

        impl From<$type> for usize {
            #[inline]
            fn from(v: $type) -> usize {
                v.as_usize()
            }
        }

        impl From<usize> for $type {
            #[inline]
            fn from(value: usize) -> Self {
                Self::from_usize(value)
            }
        }

        impl From<u32> for $type {
            #[inline]
            fn from(value: u32) -> Self {
                Self::from_u32(value)
            }
        }

        $crate::newtype_index!(
            @handle_debug
            @derives      [$($derives,)*]
            @type         [$type]
            @debug_format [$debug_format]);
    );

    // base case for handle_debug where format is custom. No Debug implementation is emitted.
    (@handle_debug
     @derives      [$($_derives:ident,)*]
     @type         [$type:ident]
     @debug_format [custom]) => ();

    // base case for handle_debug, no debug overrides found, so use default
    (@handle_debug
     @derives      []
     @type         [$type:ident]
     @debug_format [$debug_format:tt]) => (
        impl ::std::fmt::Debug for $type {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(fmt, $debug_format, self.as_u32())
            }
        }
    );

    // Debug is requested for derive, don't generate any Debug implementation.
    (@handle_debug
     @derives      [Debug, $($derives:ident,)*]
     @type         [$type:ident]
     @debug_format [$debug_format:tt]) => ();

    // It's not Debug, so just pop it off the front of the derives stack and check the rest.
    (@handle_debug
     @derives      [$_derive:ident, $($derives:ident,)*]
     @type         [$type:ident]
     @debug_format [$debug_format:tt]) => (
        $crate::newtype_index!(
            @handle_debug
            @derives      [$($derives,)*]
            @type         [$type]
            @debug_format [$debug_format]);
    );

    // Append comma to end of derives list if it's missing
    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   derive [$($derives:ident),*]
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          derive [$($derives,)*]
                          $($tokens)*);
    );

    // By not including the @derives marker in this list nor in the default args, we can force it
    // to come first if it exists. When encodable is custom, just use the derives list as-is.
    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   derive [$($derives:ident,)+]
                   ENCODABLE = custom
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @attrs        [$(#[$attrs])*]
            @derives      [$($derives,)+]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );

    // By not including the @derives marker in this list nor in the default args, we can force it
    // to come first if it exists. When encodable isn't custom, add serialization traits by default.
    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   derive [$($derives:ident,)+]
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      [$($derives,)+]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
        $crate::newtype_index!(@serializable $type);
    );

    // The case where no derives are added, but encodable is overridden. Don't
    // derive serialization traits
    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   ENCODABLE = custom
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      []
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );

    // The case where no derives are added, add serialization derives by default
    (@attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      []
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
        $crate::newtype_index!(@serializable $type);
    );

    (@serializable $type:ident) => (
        // impl<D: ::rustc_serialize::Decoder> ::rustc_serialize::Decodable<D> for $type {
        //     fn decode(d: &mut D) -> Self {
        //         Self::from_u32(d.read_u32())
        //     }
        // }
        // impl<E: ::rustc_serialize::Encoder> ::rustc_serialize::Encodable<E> for $type {
        //     fn encode(&self, e: &mut E) -> Result<(), E::Error> {
        //         e.emit_u32(self.private)
        //     }
        // }
    );

    // Rewrite final without comma to one that includes comma
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   $name:ident = $constant:expr) => (
        $crate::newtype_index!(
            @derives      [$($derives,)*]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $name = $constant,);
    );

    // Rewrite final const without comma to one that includes comma
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   $(#[doc = $doc:expr])*
                   const $name:ident = $constant:expr) => (
        $crate::newtype_index!(
            @derives      [$($derives,)*]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $(#[doc = $doc])* const $name = $constant,);
    );

    // Replace existing default for max
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$_max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   MAX = $max:expr,
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      [$($derives,)*]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );

    // Replace existing default for debug_format
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$_debug_format:tt]
                   DEBUG_FORMAT = $debug_format:tt,
                   $($tokens:tt)*) => (
        $crate::newtype_index!(
            @derives      [$($derives,)*]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );

    // Assign a user-defined constant
    (@derives      [$($derives:ident,)*]
     @attrs        [$(#[$attrs:meta])*]
     @type         [$type:ident]
     @max          [$max:expr]
     @vis          [$v:vis]
     @debug_format [$debug_format:tt]
                   $(#[doc = $doc:expr])*
                   const $name:ident = $constant:expr,
                   $($tokens:tt)*) => (
        $(#[doc = $doc])*
        $v const $name: $type = $type::from_u32($constant);
        $crate::newtype_index!(
            @derives      [$($derives,)*]
            @attrs        [$(#[$attrs])*]
            @type         [$type]
            @max          [$max]
            @vis          [$v]
            @debug_format [$debug_format]
                          $($tokens)*);
    );
}

#[cfg(debug_assertions)]
pub use newtype_index;

// #[macro_export]
// macro_rules! orc_index {
//     ($name:ident) => {
//         #[cfg(debug_assertions)]
//         $crate::newtype_index! {
//             pub struct $name {
//                 DEBUG_FORMAT = "{}"
//             }
//         }
//         #[cfg(not(debug_assertions))]
//         rustc_index::newtype_index! {
//             pub struct $name {
//                 DEBUG_FORMAT = "{}"
//             }
//         }
//         impl Default for $name {
//             fn default() -> Self {
//                 Self::from_u32(0)
//             }
//         }
//         $crate::petgraph_index! {$name}
//     };
// }

// pub use orc_index;
