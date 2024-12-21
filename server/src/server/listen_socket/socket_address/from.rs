use super::SocketAddress;
use linux::{
    netinet::r#in::{in6_addr, in_addr, sockaddr_in, sockaddr_in6},
    sys::socket::{AF_INET, AF_INET6},
};
use std::net::SocketAddr;

impl From<SocketAddr> for SocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => SocketAddress::V4(sockaddr_in {
                family: AF_INET as _,
                port: addr.port().to_be(),
                addr: in_addr {
                    addr: addr.ip().to_bits().to_be(),
                },
                ..Default::default()
            }),
            SocketAddr::V6(addr) => SocketAddress::V6(sockaddr_in6 {
                family: AF_INET6 as _,
                port: addr.port().to_be(),
                flow_info: addr.flowinfo().to_be(),
                addr: in6_addr {
                    addr: addr.ip().octets(),
                },
                scope_id: addr.scope_id().to_be(),
            }),
        }
    }
}
