use super::EPoll;
use linux::{
    sys::epoll::{epoll_ctl, EPOLL_CTL_DEL},
    try_linux,
};
use std::{ffi::c_int, ptr::null};

impl EPoll {
    /// Unregister `handle` from this epoll instance
    pub(super) fn unregister(&mut self, handle: c_int) -> linux::Result<()> {
        try_linux!(epoll_ctl(self.handle, EPOLL_CTL_DEL, handle, null()))?;
        self.count -= 1;
        Ok(())
    }
}
