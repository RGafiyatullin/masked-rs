use std::fmt;

use crate::Masked;

impl<T> fmt::Display for Masked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[masked]")
    }
}
