use super::SocketAddress;
use linux::sys::socket::sockaddr;

impl SocketAddress {
    /// Gets a pointer to the underlying [`sockaddr`]
    pub fn as_ptr(&self) -> *const sockaddr {
        match self {
            SocketAddress::V4(v4) => v4 as *const _ as _,
            SocketAddress::V6(v6) => v6 as *const _ as _,
        }
    }

    /// Gets a mutable pointer to the underlying [`sockaddr`]
    pub fn as_mut_ptr(&mut self) -> *mut sockaddr {
        match self {
            SocketAddress::V4(v4) => v4 as *mut _ as _,
            SocketAddress::V6(v6) => v6 as *mut _ as _,
        }
    }
}
