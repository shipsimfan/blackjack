use super::SocketAddress;
use std::ffi::c_int;
use win32::winsock2::{AF_INET, AF_INET6};

impl SocketAddress {
    pub fn family(&self) -> c_int {
        match self {
            SocketAddress::V4(_) => AF_INET,
            SocketAddress::V6(_) => AF_INET6,
        }
    }
}
