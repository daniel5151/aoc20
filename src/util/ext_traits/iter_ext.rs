use crate::util::gcd_lcm::GcdLcm;

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
