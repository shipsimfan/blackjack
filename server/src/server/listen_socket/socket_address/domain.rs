use super::SocketAddress;
use linux::sys::socket::{AF_INET, AF_INET6};
use std::ffi::c_int;

impl SocketAddress {
    /// Gets the address domain for this address
    pub fn domain(&self) -> c_int {
        match self {
            SocketAddress::V4(_) => AF_INET,
            SocketAddress::V6(_) => AF_INET6,
        }
    }
}
