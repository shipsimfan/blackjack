use super::Server;
use std::net::SocketAddr;

impl Server {
    /// Gets the local address of this server
    pub fn local_address(&self) -> &SocketAddr {
        self.listen_socket.local_address()
    }
}
