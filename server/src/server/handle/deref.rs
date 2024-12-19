use super::Handle;
use std::{ffi::c_int, ops::Deref};

impl Deref for Handle {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        match self {
            Handle::Unregistered(handle) => handle,
            Handle::Registered(handle) => handle,
        }
    }
}

impl AsRef<c_int> for Handle {
    fn as_ref(&self) -> &c_int {
        match self {
            Handle::Unregistered(handle) => handle,
            Handle::Registered(handle) => handle,
        }
    }
}
