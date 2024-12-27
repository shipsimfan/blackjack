use crate::messages::RefCow;

impl<'a, T: Clone> RefCow<'a, T> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> RefCow<'static, T> {
        match self {
            RefCow::Borrowed(borrowed) => RefCow::Owned(borrowed.clone()),
            RefCow::Owned(owned) => RefCow::Owned(owned),
        }
    }
}
