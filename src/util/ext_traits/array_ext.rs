use crate::util::const_generics_iterators::CartesianProduct;

/// Workaround until https://github.com/rust-lang/rust/pull/65819 is merged.
pub trait ArrayExt<T, const N: usize> {
    fn into_iter(self) -> core::array::IntoIter<T, N>;

    fn cartesian_product(self) -> CartesianProduct<T, N>
    where
        T: Iterator + Clone;
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    fn into_iter(self) -> core::array::IntoIter<T, N> {
        core::array::IntoIter::new(self)
    }

    fn cartesian_product(self) -> CartesianProduct<T, N>
    where
        T: Iterator + Clone,
    {
        CartesianProduct::new(self)
    }
}
