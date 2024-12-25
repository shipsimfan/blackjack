use std::net::SocketAddr;

mod display;

/// An error that can occur while creating the server
#[derive(Debug)]
pub enum NewServerError {
    /// An error occurred while creating the epoll instance
    EPollCreationFailed(linux::Error),

    /// Failed to create the socket to listen on
    ListenSocketCreationFailed(linux::Error),

    /// Failed to set allowing address re-use on the listen socket
    SetReuseAddressFailed(linux::Error),

    /// Failed to bind the listen socket to the requested address
    BindSocketFailed(linux::Error, SocketAddr),

    /// Failed to begin listening on the socket
    ListenSocketFailed(linux::Error),

    /// Failed to set the socket to non-blocking
    SetNonBlockingFailed(linux::Error),

    /// Failed to get the local address of the listen socket
    GetLocalAddressFailed(linux::Error),

    /// Failed to register the listen socket with the e-poll instance
    RegisterListenSocketFailed(linux::Error),
}

impl std::error::Error for NewServerError {}
