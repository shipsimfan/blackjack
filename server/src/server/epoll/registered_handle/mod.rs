use super::EPoll;
use std::{cell::RefCell, ffi::c_int, rc::Rc};

mod deref;
mod drop;
mod new;

/// A handle that has been register with an [`EPoll`]
pub struct RegisteredHandle {
    /// The handle which has been registered
    handle: c_int,

    /// The epoll it has been registered to
    epoll: Rc<RefCell<EPoll>>,
}
