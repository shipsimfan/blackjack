use crate::Connection;
use win32::{winsock2::closesocket, CloseHandle};

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { closesocket(self.handle) };
        unsafe { CloseHandle(self.event) };
    }
}
