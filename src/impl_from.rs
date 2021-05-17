use crate::Masked;

impl<T> From<T> for Masked<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
