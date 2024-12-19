use super::ListenSocket;
use linux::{sys::socket::accept, try_linux, unistd::close};
use std::ptr::null_mut;

impl ListenSocket {
    /// Accepts a client on this socket
    pub fn accept(&mut self) -> linux::Result<()> {
        let handle = try_linux!(accept(*self.handle, null_mut(), null_mut()))?;

        unsafe { close(handle) };
        Ok(())
    }
}
