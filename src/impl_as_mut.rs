
use crate::Masked;

impl<T> AsMut<T> for Masked<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}