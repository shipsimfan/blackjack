use super::{ListenSocket, SocketAddress};
use crate::server::{handle::Handle, EPoll, NewServerError};
use linux::{
    sys::{
        epoll::EPOLLIN,
        socket::{bind, listen, socket, SOCK_STREAM, SOMAXCONN},
    },
    try_linux,
};
use std::{cell::RefCell, net::SocketAddr, rc::Rc};

impl ListenSocket {
    /// Open a socket for listening on `addr`
    pub fn listen(addr: SocketAddr, epoll: Rc<RefCell<EPoll>>) -> Result<Self, NewServerError> {
        // Convert the socket address
        let linux_addr = SocketAddress::from(addr);

        // Create the listen socket
        let mut handle = Handle::Unregistered(
            try_linux!(socket(linux_addr.domain(), SOCK_STREAM, 0))
                .map_err(NewServerError::ListenSocketCreationFailed)?,
        );

        // Bind the socket to the address
        try_linux!(bind(*handle, linux_addr.as_ptr(), linux_addr.len()))
            .map_err(|error| NewServerError::BindSocketFailed(error, addr))?;

        // Begin listening for clients
        try_linux!(listen(*handle, SOMAXCONN)).map_err(NewServerError::ListenSocketFailed)?;

        // Register with e-poll
        handle
            .register(epoll, Self::ID, EPOLLIN)
            .map_err(NewServerError::RegisterListenSocketFailed)?;

        Ok(ListenSocket { handle })
    }
}
