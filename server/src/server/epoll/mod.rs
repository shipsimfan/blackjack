use std::ffi::c_int;

mod registered_handle;

mod drop;
mod event;
mod new;
mod register;
mod unregister;
mod wait;

pub use event::EPollEvent;
pub use registered_handle::RegisteredHandle;

/// The manager for handling multiple sockets on one thread
pub(super) struct EPoll {
    /// The handle to the epoll structure
    handle: c_int,

    /// The current number of registered handles
    count: usize,
}
