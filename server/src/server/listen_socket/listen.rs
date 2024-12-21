use super::{ListenSocket, SocketAddress};
use crate::server::{handle::Handle, EPoll, NewServerError};
use linux::{
    fcntl::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK},
    netinet::r#in::{sockaddr_in, sockaddr_in6},
    sys::{
        epoll::EPOLLIN,
        socket::{bind, getsockname, listen, socket, AF_INET, AF_INET6, SOCK_STREAM, SOMAXCONN},
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

        // Set non-blocking
        let mut flags =
            try_linux!(fcntl(*handle, F_GETFL, 0)).map_err(NewServerError::SetNonBlockingFailed)?;
        flags |= O_NONBLOCK;
        try_linux!(fcntl(*handle, F_SETFL, flags)).map_err(NewServerError::SetNonBlockingFailed)?;

        // Get the local address
        let mut local_address = match linux_addr.domain() {
            AF_INET => SocketAddress::V4(sockaddr_in::default()),
            AF_INET6 => SocketAddress::V6(sockaddr_in6::default()),
            _ => unreachable!(),
        };
        let mut length = local_address.len();
        try_linux!(getsockname(
            *handle,
            local_address.as_mut_ptr(),
            &mut length
        ))
        .map_err(NewServerError::GetLocalAddressFailed)?;

        // Register with e-poll
        handle
            .register(epoll, Self::ID, EPOLLIN)
            .map_err(NewServerError::RegisterListenSocketFailed)?;

        Ok(ListenSocket {
            handle,
            local_address: local_address.into(),
        })
    }
}
