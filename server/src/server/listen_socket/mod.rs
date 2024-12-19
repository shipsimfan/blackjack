use super::Handle;
use socket_address::SocketAddress;

mod accept;
mod listen;
mod socket_address;

/// A socket for listening for client connections
pub(super) struct ListenSocket {
    /// The handle to the socket
    handle: Handle,
}

impl ListenSocket {
    /// The id for the listen socket used in e-poll
    pub const ID: u64 = u64::MAX;
}
