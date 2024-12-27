use crate::messages::RefCow;

impl<'a, T: Clone> RefCow<'a, T> {
    /// Unwraps the underlying value into an owned version
    pub fn unwrap(self) -> T {
        match self {
            RefCow::Borrowed(borrowed) => borrowed.clone(),
            RefCow::Owned(owned) => owned,
        }
    }
}
