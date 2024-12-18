use super::SocketAddress;
use linux::{
    netinet::r#in::{sockaddr_in, sockaddr_in6},
    sys::socket::socklen_t,
};

impl SocketAddress {
    /// Gets the length of the underlying [`sockaddr`](linux::netinet::r#in::sockaddr)
    pub fn len(&self) -> socklen_t {
        match self {
            SocketAddress::V4(_) => std::mem::size_of::<sockaddr_in>() as _,
            SocketAddress::V6(_) => std::mem::size_of::<sockaddr_in6>() as _,
        }
    }
}
