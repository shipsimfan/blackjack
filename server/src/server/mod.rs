use epoll::{EPoll, RegisteredHandle};
use listen_socket::ListenSocket;
use new::NewServerError;
use std::{cell::RefCell, rc::Rc};

mod epoll;
mod listen_socket;

mod new;

/// The server infrastructure
pub struct Server {
    /// The manager for polling mulitple sockets
    epoll: Rc<RefCell<EPoll>>,

    /// The socket for listening for incoming clients
    listen_socket: ListenSocket,
}
