//! Useful types and traits for solving AoC style questions.
//!
//! Rome wasn't built in a day, and neither was this prelude. Expect this to
//! keep growing as more questions get solved.

// useful std stuff
pub use std::collections::*;
pub use std::io::prelude::*;

pub use iter_to_array::*; // collect into fixed size arrays
pub use itertools::{Itertools, MinMaxResult};

pub use crate::DynResult;

pub use crate::util::ext_traits::*;
pub use crate::util::gcd_lcm::GcdLcm;

/// misc useful AoC things
pub mod aoc {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

/// More compact than the default `{:#?}` output, while still printing each
/// (k, v) on a new line.
#[macro_export]
macro_rules! dbg_map {
    ($map:expr) => {
        for (k, v) in $map.iter().sorted_by(|(k1, _), (k2, _)| k1.cmp(k2)) {
            eprintln!("{:?}: {:?}", k, v);
        }
    };
}
