use super::SocketAddress;
use win32::winsock2::sockaddr;

impl SocketAddress {
    /// Gets the socket address a pointer
    pub fn as_ptr(&self) -> *const sockaddr {
        match self {
            SocketAddress::V4(addr) => addr as *const _ as _,
            SocketAddress::V6(addr) => addr as *const _ as _,
        }
    }
}
