#[derive(
    ::serde::Serialize,
    ::serde::Deserialize,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct Masked<T>(pub(crate) T);

impl<T> Masked<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Masked<T>
where
    T: AsRef<str>,
{
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
