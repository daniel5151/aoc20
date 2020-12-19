use crate::util::const_generics_iterators::Combinations;

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
    fn combinations<const N: usize>(&self) -> Combinations<'_, T, N>;
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

    fn combinations<const N: usize>(&self) -> Combinations<'_, T, N> {
        Combinations::new(self.as_ref())
    }
}
