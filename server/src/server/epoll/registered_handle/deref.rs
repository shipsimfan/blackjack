use super::RegisteredHandle;
use std::{ffi::c_int, ops::Deref};

impl Deref for RegisteredHandle {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl AsRef<c_int> for RegisteredHandle {
    fn as_ref(&self) -> &c_int {
        &self.handle
    }
}
