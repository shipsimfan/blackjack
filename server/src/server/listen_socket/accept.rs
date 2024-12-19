use super::ListenSocket;
use crate::server::Handle;
use linux::{sys::socket::accept, try_linux};
use std::ptr::null_mut;

impl ListenSocket {
    /// Accepts a client on this socket
    pub fn accept(&mut self) -> linux::Result<Handle> {
        try_linux!(accept(*self.handle, null_mut(), null_mut())).map(Handle::Unregistered)
    }
}
