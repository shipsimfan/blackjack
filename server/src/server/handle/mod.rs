use super::RegisteredHandle;
use std::ffi::c_int;

mod deref;
mod drop;
mod register;

/// A handle to a socket
pub(super) enum Handle {
    /// The handle is unregistered
    Unregistered(c_int),

    /// The handle is registered
    Registered(RegisteredHandle),
}
