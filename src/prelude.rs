//! Useful types and traits for solving AoC style questions.
//!
//! Rome wasn't built in a day, and neither was this prelude. Expect this to
//! keep growing as more questions get solved.

// useful std stuff
pub use std::collections::*;
pub use std::io::prelude::*;

// useful external libraries
pub use itertools::Itertools;

pub use crate::DynResult;

pub trait GcdLcm {
    /// Find the Greatest Common Divisor between two integers
    fn gcd(self, other: Self) -> Self;
    /// Find the Lowest Common Multiple between two integers
    fn lcm(self, other: Self) -> Self;
}

macro_rules! gcdlcm_impl {
        ($($type:ty),*) => ($(
            impl GcdLcm for $type {
                fn gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = if self > other {
                        (self, other)
                    } else {
                        (other, self)
                    };

                    while b != 0 {
                        let r = a % b;
                        a = b;
                        b = r;
                    }

                    a
                }

                fn lcm(self, other: Self) -> Self {
                    self * other / self.gcd(other)
                }
            }
        )*)
    }

gcdlcm_impl! { u8, u16, u32, u64, u128, usize }

pub trait SliceExt<T> {
    /// An implementation of
    /// [`core::slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point)
    /// that includes the index of the current element in the predicate.
    ///
    /// Returns the index of the partition point according to the given
    /// predicate (the index of the first element of the second
    /// partition).
    fn partition_point_enumerated(&self, pred: impl FnMut(usize, &T) -> bool) -> usize;
}

impl<T, S> SliceExt<T> for S
where
    S: AsRef<[T]>,
{
    fn partition_point_enumerated(&self, mut pred: impl FnMut(usize, &T) -> bool) -> usize {
        let s = self.as_ref();

        let mut left = 0;
        let mut right = s.len();

        while left != right {
            let mid = left + (right - left) / 2;
            // SAFETY: When `left < right`, `left <= mid < right`.
            // Therefore `left` always increases and `right` always decreases,
            // and either of them is selected. In both cases `left <= right` is
            // satisfied. Therefore if `left < right` in a step, `left <= right`
            // is satisfied in the next step. Therefore as long as `left != right`,
            // `0 <= left < right <= len` is satisfied and if this case
            // `0 <= mid < len` is satisfied too.
            let value = unsafe { s.get_unchecked(mid) };
            if pred(mid, value) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

/// Useful AOC things
pub mod aoc {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
