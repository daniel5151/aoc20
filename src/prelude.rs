//! Useful types and traits for solving AoC style questions.
//!
//! Rome wasn't built in a day, and neither was this prelude. Expect this to
//! keep growing as more questions get solved.

// useful std stuff
pub use std::collections::*;
pub use std::io::prelude::*;

// useful external libraries
pub use itertools::{Itertools, MinMaxResult};

pub use crate::DynResult;

pub use crate::vm;

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

    /// Return an iterator over all combinations of size N in the slice.
    fn combinations_const<const N: usize>(&self) -> CombinationsConst<'_, T, N>;
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

    fn combinations_const<const N: usize>(&self) -> CombinationsConst<'_, T, N> {
        CombinationsConst::new(self.as_ref())
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

pub struct CombinationsConst<'a, T, const N: usize> {
    arr: &'a [T],
    idx: [usize; N],
}

impl<'a, T, const N: usize> CombinationsConst<'a, T, N> {
    fn new(arr: &'a [T]) -> Self {
        let mut idx = [0; { N }];
        for (i, idx) in idx.iter_mut().enumerate() {
            *idx = i
        }

        CombinationsConst { arr, idx }
    }
}

impl<'a, T, const N: usize> Iterator for CombinationsConst<'a, T, N> {
    type Item = [&'a T; N];

    // https://stackoverflow.com/questions/5076695/how-can-i-iterate-through-every-possible-combination-of-n-playing-cards
    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 || N > self.arr.len() {
            return None;
        }

        let mut n = self.arr.len();
        for i in (0..=(N - 1)).rev() {
            n -= 1;
            if self.idx[i] < n {
                self.idx[i] += 1;
                for j in (i + 1)..N {
                    self.idx[j] = self.idx[j - 1] + 1;
                }

                let mut out = [&self.arr[0]; { N }];
                for (e, v) in out.iter_mut().zip(self.idx.iter().copied()) {
                    *e = &self.arr[v]
                }

                return Some(out);
            }
        }

        None
    }
}
