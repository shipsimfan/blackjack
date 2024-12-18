use super::{EPoll, RegisteredHandle};
use std::{cell::RefCell, ffi::c_int, rc::Rc};

impl RegisteredHandle {
    /// Create a new [`RegisteredHandle`]
    pub(in crate::server::epoll) fn new(handle: c_int, epoll: Rc<RefCell<EPoll>>) -> Self {
        RegisteredHandle { handle, epoll }
    }
}
