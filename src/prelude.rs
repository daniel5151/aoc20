//! Useful types and traits for solving AoC style questions.
//!
//! Rome wasn't built in a day, and neither was this prelude. Expect this to
//! keep growing as more questions get solved.

// useful std stuff
pub use std::collections::*;
pub use std::io::prelude::*;

// useful external libraries
pub use iter_to_array::*;
pub use itertools::{Itertools, MinMaxResult};

pub use crate::DynResult;

pub trait GcdLcm: Sized + num_traits::PrimInt {
    /// Find the Greatest Common Divisor between two numbers
    fn gcd(self, other: Self) -> Self {
        let (mut a, mut b) = if self > other {
            (self, other)
        } else {
            (other, self)
        };

        while !b.is_zero() {
            let r = a % b;
            a = b;
            b = r;
        }

        a
    }

    /// Find the Lowest Common Multiple between two numbers
    fn lcm(self, other: Self) -> Self {
        self * other / self.gcd(other)
    }

    /// Finds the Lowest Common Multiple of a list of numbers.
    ///
    /// Returns zero if there aren't any numbers in the list.
    fn lcm_list(nums: impl IntoIterator<Item = Self>) -> Self {
        let mut nums = nums.into_iter();

        let first = match nums.next() {
            None => return Self::zero(),
            Some(n) => n,
        };

        nums.fold(first, |a, n| (n * a) / (n.gcd(a)))
    }

    /// Finds the Greatest Common Divisor of a list of numbers.
    ///
    /// Returns zero if there aren't any numbers in the list.
    fn gcd_list(nums: impl IntoIterator<Item = Self>) -> Self {
        let mut nums = nums.into_iter();

        let first = match nums.next() {
            None => return Self::zero(),
            Some(n) => n,
        };

        nums.fold(first, |a, n| n.gcd(a))
    }
}

macro_rules! gcdlcm_impl {
    ($($type:ty),*) => ($(
        impl GcdLcm for $type {}
    )*)
}

gcdlcm_impl! { u8, u16, u32, u64, u128, usize }

pub trait IterExt<T>: Iterator<Item = T> {
    fn lcm(self) -> T
    where
        T: GcdLcm;

    fn gcd(self) -> T
    where
        T: GcdLcm;
}

impl<T, I> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn lcm(self) -> T
    where
        T: GcdLcm,
    {
        GcdLcm::lcm_list(self)
    }

    fn gcd(self) -> T
    where
        T: GcdLcm,
    {
        GcdLcm::gcd_list(self)
    }
}

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
    fn combinations_const<const N: usize>(&self) -> Combinations<'_, T, N>;
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

    fn combinations_const<const N: usize>(&self) -> Combinations<'_, T, N> {
        Combinations::new(self.as_ref())
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

pub struct Combinations<'a, T, const N: usize> {
    arr: &'a [T],
    idx: [usize; N],
    // HACK: the implementation I used is from C++, which splits retrieving the element and
    // advancing the iterator. Rust merges the two operations, and being lazy, I've decided to just
    // work around the problem instead of solving it properly.
    first: bool,
}

impl<'a, T, const N: usize> Combinations<'a, T, N> {
    fn new(arr: &'a [T]) -> Self {
        let mut idx = [0; { N }];
        for (i, idx) in idx.iter_mut().enumerate() {
            *idx = i
        }

        Combinations {
            arr,
            idx,
            first: true,
        }
    }

    fn output(&mut self) -> [&'a T; N] {
        let mut out = [&self.arr[0]; { N }];
        for (e, v) in out.iter_mut().zip(self.idx.iter().copied()) {
            *e = &self.arr[v]
        }
        out
    }
}

impl<'a, T, const N: usize> Iterator for Combinations<'a, T, N> {
    type Item = [&'a T; N];

    // https://stackoverflow.com/questions/5076695/how-can-i-iterate-through-every-possible-combination-of-n-playing-cards
    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 || N > self.arr.len() {
            return None;
        }

        if self.first {
            self.first = false;
            return Some(self.output());
        }

        let mut n = self.arr.len();
        for i in (0..=(N - 1)).rev() {
            n -= 1;
            if self.idx[i] < n {
                self.idx[i] += 1;
                for j in (i + 1)..N {
                    self.idx[j] = self.idx[j - 1] + 1;
                }

                return Some(self.output());
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct CartesianProduct<I, const N: usize>
where
    I: Iterator,
{
    first: bool,
    base: [I; N],
    cur_i: [I; N],
    cur_e: [I::Item; N],
}

impl<I, const N: usize> CartesianProduct<I, N>
where
    I: Iterator + Clone,
{
    pub fn new(base: [I; N]) -> CartesianProduct<I, N> {
        use core::mem::MaybeUninit;

        pub fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
            // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
            unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
        }

        unsafe fn assume_init_arr<T, const N: usize>(arr: [MaybeUninit<T>; N]) -> [T; N] {
            core::mem::transmute_copy(&arr)
        }

        let mut cur_i = base.clone();
        let mut cur_e_uninit: [MaybeUninit<I::Item>; N] = uninit_array();
        for (e, i) in cur_e_uninit.iter_mut().zip(cur_i.iter_mut()) {
            unsafe {
                e.as_mut_ptr()
                    .write(i.next().expect("iterator must return at least one element"))
            };
        }

        let cur_e = unsafe { assume_init_arr(cur_e_uninit) };

        CartesianProduct {
            first: true,
            base,
            cur_i,
            cur_e,
        }
    }
}

impl<I, const N: usize> Iterator for CartesianProduct<I, N>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<[I::Item; N]> {
        if self.first {
            self.first = false;
            return Some(self.cur_e.clone());
        }

        let mut could_iter = false;
        for i in (0..N).rev() {
            if let Some(e) = self.cur_i[i].next() {
                self.cur_e[i] = e;
                could_iter = true;
                break;
            } else {
                for j in i..N {
                    self.cur_i[j] = self.base[j].clone();
                    self.cur_e[j] = self.cur_i[j].next().unwrap();
                }
            }
        }

        if !could_iter {
            return None;
        }

        Some(self.cur_e.clone())
    }
}

/// Workaround until https://github.com/rust-lang/rust/pull/65819 is merged.
pub trait ArrayIntoIterExt<T, const N: usize> {
    fn into_iter(self) -> core::array::IntoIter<T, N>;
}

impl<T, const N: usize> ArrayIntoIterExt<T, N> for [T; N] {
    fn into_iter(self) -> core::array::IntoIter<T, N> {
        core::array::IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combo_const() {
        let test = [1, 2, 3];
        assert_eq!(
            test.combinations_const().collect::<Vec<_>>(),
            [[&1, &2], [&1, &3], [&2, &3]]
        );
    }
}
