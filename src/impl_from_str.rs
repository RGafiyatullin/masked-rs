use std::str;

use crate::Masked;

impl<T> str::FromStr for Masked<T>
where
    T: str::FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(Self)
    }
}
