use super::ListenSocket;
use std::net::SocketAddr;

impl ListenSocket {
    /// Gets the local address of this socket
    pub fn local_address(&self) -> &SocketAddr {
        &self.local_address
    }
}
