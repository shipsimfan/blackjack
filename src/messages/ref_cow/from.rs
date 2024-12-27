use crate::messages::RefCow;

impl<'a, T> From<T> for RefCow<'a, T> {
    fn from(value: T) -> Self {
        RefCow::Owned(value)
    }
}

impl<'a, T> From<&'a T> for RefCow<'a, T> {
    fn from(value: &'a T) -> Self {
        RefCow::Borrowed(value)
    }
}
