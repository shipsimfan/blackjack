use linux::netinet::r#in::{sockaddr_in, sockaddr_in6};

mod as_ptr;
mod domain;
mod from;
mod into;
mod len;

/// An address for a socket
pub enum SocketAddress {
    /// IP Version 4 address
    V4(sockaddr_in),

    /// IP Version 6 address
    V6(sockaddr_in6),
}
