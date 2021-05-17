use std::fmt;

use crate::Masked;

impl<T> fmt::Debug for Masked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Masked<{}>", std::any::type_name::<T>())
    }
}
