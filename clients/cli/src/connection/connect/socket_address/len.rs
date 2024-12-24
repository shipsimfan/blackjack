use super::SocketAddress;
use win32::winsock2::{sockaddr_in, sockaddr_in6};

impl SocketAddress {
    /// Gets the length of the contained socket address
    pub fn len(&self) -> usize {
        match self {
            SocketAddress::V4(_) => std::mem::size_of::<sockaddr_in>(),
            SocketAddress::V6(_) => std::mem::size_of::<sockaddr_in6>(),
        }
    }
}
