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
