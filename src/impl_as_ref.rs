use crate::Masked;

impl<T> AsRef<T> for Masked<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
