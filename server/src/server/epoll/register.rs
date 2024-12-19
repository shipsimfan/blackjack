use super::{registered_handle::RegisteredHandle, EPoll};
use linux::{
    sys::epoll::{epoll_ctl, epoll_event, EPOLL_CTL_ADD},
    try_linux,
};
use std::{cell::RefCell, ffi::c_int, rc::Rc};

impl EPoll {
    /// Register `handle` with this epoll instance
    pub fn register_handle(
        this: Rc<RefCell<Self>>,
        handle: c_int,
        id: u64,
        events: u32,
    ) -> linux::Result<RegisteredHandle> {
        let event = epoll_event {
            events,
            data: linux::sys::epoll::epoll_data_t { u64: id },
        };

        let mut this_ref = this.borrow_mut();

        try_linux!(epoll_ctl(this_ref.handle, EPOLL_CTL_ADD, handle, &event))?;
        this_ref.count += 1;

        drop(this_ref);

        Ok(RegisteredHandle::new(handle, this))
    }
}
