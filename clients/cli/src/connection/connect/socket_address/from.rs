use super::SocketAddress;
use std::net::SocketAddr;
use win32::winsock2::{sockaddr_in, sockaddr_in6, AF_INET, AF_INET6};

impl From<SocketAddr> for SocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => SocketAddress::V4(sockaddr_in {
                family: AF_INET as _,
                port: addr.port().to_be(),
                addr: addr.ip().to_bits().to_be(),
                zero: [0; 8],
            }),
            SocketAddr::V6(addr) => SocketAddress::V6(sockaddr_in6 {
                family: AF_INET6 as _,
                port: addr.port().to_be(),
                flowinfo: addr.flowinfo().to_be(),
                addr: addr.ip().octets(),
                scope_id: addr.scope_id().to_be(),
            }),
        }
    }
}
