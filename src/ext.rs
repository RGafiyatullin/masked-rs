
use crate::Masked;

pub trait MaskedExt: Sized {
    fn masked(self) -> Masked<Self> {
        self.into()
    }
}

impl<T> MaskedExt for T {}
