use client_socket::ClientSocket;
use epoll::{EPoll, RegisteredHandle};
use handle::Handle;
use listen_socket::ListenSocket;
use new::NewServerError;
use std::{cell::RefCell, rc::Rc};

mod client_socket;
mod epoll;
mod handle;
mod listen_socket;

mod new;
mod run;

/// The server infrastructure
pub struct Server {
    /// The manager for polling mulitple sockets
    epoll: Rc<RefCell<EPoll>>,

    /// The socket for listening for incoming clients
    listen_socket: ListenSocket,

    /// The currently connected clients
    clients: Box<[Option<ClientSocket>]>,
}
