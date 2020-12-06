#![allow(clippy::unreadable_literal)]
#![allow(clippy::type_complexity)]

/// Catch-all error type (works with anything that implements std::error::Error)
pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod prelude {
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
}

// Utility macro to make adding new days a breeze
macro_rules! days {
    ($($day:ident),* $(,)*) => {
        $(mod $day;)*

        fn route_day(day: &str, question: &str, input: &str, other_args: &[&str]) -> DynResult<()> {
            let day = format!("day{}", day);

            match day.as_str() {
                $(stringify!($day) => match question {
                    "1" => println!("Answer: {:?}", $day::q1(input, other_args)?),
                    "2" => println!("Answer: {:?}", $day::q2(input, other_args)?),
                    _ => return Err("Unknown question".into()),
                })*
                _ => return Err("Unknown day".into()),
            }

            Ok(())
        }
    };
}

days! {
    day1,
    day2,
    day3,
    day4,
    day5,
}

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args = args.iter().map(String::as_str).collect::<Vec<&str>>();

    let (day, question) = match (args.get(1), args.get(2)) {
        (None, _) | (_, None) => return Err("Must specify day and question (e.g: 3 1)".into()),
        (Some(d), Some(q)) => (d, q),
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = std::path::Path::new(&input_path);

    let mut input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Could not open {}: {}", input_path.to_string_lossy(), e))?;
    input.truncate(input.trim_end().len());

    route_day(day, question, &input, &args[3..])
}
