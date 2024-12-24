use win32::winsock2::{sockaddr_in, sockaddr_in6};

mod as_ptr;
mod family;
mod from;
mod len;

/// An address to connect to
pub(super) enum SocketAddress {
    /// IPv4 address
    V4(sockaddr_in),

    /// IPv6 address
    V6(sockaddr_in6),
}
