use super::{EPoll, ListenSocket, Server};
use std::net::SocketAddr;

mod error;

pub use error::NewServerError;

impl Server {
    /// Create a new [`Server`]
    pub fn new(addr: SocketAddr, max_clients: usize) -> Result<Self, NewServerError> {
        let epoll = EPoll::new(max_clients)?;

        let listen_socket = ListenSocket::listen(addr, epoll.clone())?;

        Ok(Server {
            epoll,
            listen_socket,
        })
    }
}
