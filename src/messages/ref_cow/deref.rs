use crate::messages::RefCow;
use std::ops::Deref;

impl<'a, T> Deref for RefCow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            RefCow::Borrowed(borrowed) => *borrowed,
            RefCow::Owned(owned) => owned,
        }
    }
}

impl<'a, T> AsRef<T> for RefCow<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            RefCow::Borrowed(borrowed) => *borrowed,
            RefCow::Owned(owned) => owned,
        }
    }
}
