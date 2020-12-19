/// Constructed through the `SliceExt::combinations()` method.
pub struct Combinations<'a, T, const N: usize> {
    arr: &'a [T],
    idx: [usize; N],
    // HACK: the implementation I used is from C++, which splits retrieving the element and
    // advancing the iterator. Rust merges the two operations, and being lazy, I've decided to just
    // work around the problem instead of solving it properly.
    first: bool,
}

impl<'a, T, const N: usize> Combinations<'a, T, N> {
    pub fn new(arr: &'a [T]) -> Self {
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

#[cfg(test)]
mod combinations_tests {
    use crate::util::ext_traits::*;

    #[test]
    fn combinations_const() {
        let test = [1, 2, 3];
        assert_eq!(
            test.combinations().collect::<Vec<_>>(),
            [[&1, &2], [&1, &3], [&2, &3]]
        );
    }
}

/// Constructed through the `ArrayExt::cartesian_product()` method.
#[derive(Clone)]
pub struct CartesianProduct<I, const N: usize>
where
    I: Iterator,
{
    // HACK: this could probably be rewritten to not require a "first" boolean
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

#[cfg(test)]
mod cartesian_product_tests {
    use crate::util::ext_traits::*;

    #[test]
    fn cartesian_product() {
        let a = [1, 2, 3];
        let b = [4, 5];
        let prod = [a.iter().copied(), b.iter().copied()];
        assert_eq!(
            prod.cartesian_product().collect::<Vec<_>>(),
            [[1, 4], [1, 5], [2, 4], [2, 5], [3, 4], [3, 5]]
        );
    }
}
