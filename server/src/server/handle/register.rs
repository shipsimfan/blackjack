use super::Handle;
use crate::server::EPoll;
use std::{cell::RefCell, rc::Rc};

impl Handle {
    pub fn register(
        &mut self,
        epoll: Rc<RefCell<EPoll>>,
        id: u64,
        events: u32,
    ) -> linux::Result<()> {
        let handle = match self {
            Handle::Unregistered(handle) => *handle,
            Handle::Registered(_) => panic!("Cannot register an already registered handle!"),
        };

        *self = Handle::Registered(EPoll::register_handle(epoll, handle, id, events)?);
        Ok(())
    }
}
