use super::{EPoll, ListenSocket, Server};
use std::{cell::RefCell, net::SocketAddr, rc::Rc};

mod error;

pub use error::NewServerError;

impl Server {
    /// Create a new [`Server`]
    pub fn new(addr: SocketAddr, max_clients: usize) -> Result<Self, NewServerError> {
        let epoll = EPoll::new(max_clients)?;

        let listen_socket = ListenSocket::listen(addr, epoll.clone())?;

        let mut clients = Vec::with_capacity(max_clients);
        for _ in 0..max_clients {
            clients.push(None);
        }

        Ok(Server {
            epoll,
            listen_socket,
            clients: clients.into_boxed_slice(),
            clients_to_disconnect: Rc::new(RefCell::new(Vec::with_capacity(max_clients))),
        })
    }
}
