use super::EPoll;
use crate::server::NewServerError;
use linux::{sys::epoll::epoll_create, try_linux};
use std::{cell::RefCell, ffi::c_int, rc::Rc};

impl EPoll {
    /// Create a new e-poll instance
    pub fn new(max_clients: usize) -> Result<Rc<RefCell<Self>>, NewServerError> {
        let handle = try_linux!(epoll_create(max_clients as c_int + 1))
            .map_err(NewServerError::EPollCreationFailed)?;

        Ok(Rc::new(RefCell::new(EPoll { handle, count: 0 })))
    }
}
