use super::{ListenSocket, SocketAddress};
use crate::server::{EPoll, NewServerError};
use linux::{
    sys::{
        epoll::EPOLLIN,
        socket::{bind, listen, socket, SOCK_STREAM, SOMAXCONN},
    },
    try_linux,
    unistd::close,
};
use std::{cell::RefCell, net::SocketAddr, rc::Rc};

impl ListenSocket {
    /// Open a socket for listening on `addr`
    pub fn listen(addr: SocketAddr, epoll: Rc<RefCell<EPoll>>) -> Result<Self, NewServerError> {
        // Convert the socket address
        let linux_addr = SocketAddress::from(addr);

        // Create the listen socket
        let handle = try_linux!(socket(linux_addr.domain(), SOCK_STREAM, 0))
            .map_err(NewServerError::ListenSocketCreationFailed)?;

        // Register it with e-poll
        let handle = EPoll::register_handle(epoll, handle, Self::ID, EPOLLIN).map_err(|error| {
            unsafe { close(handle) };
            NewServerError::RegisterListenSocketFailed(error)
        })?;

        // Create the socket so future errors auto close
        let socket = ListenSocket { handle };

        // Bind the socket to the address
        try_linux!(bind(*socket.handle, linux_addr.as_ptr(), linux_addr.len()))
            .map_err(|error| NewServerError::BindSocketFailed(error, addr))?;

        // Begin listening for clients
        try_linux!(listen(*socket.handle, SOMAXCONN))
            .map_err(NewServerError::ListenSocketFailed)?;

        Ok(socket)
    }
}
