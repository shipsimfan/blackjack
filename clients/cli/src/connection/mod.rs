use win32::winsock2::SOCKET;

mod drop;

/// A connection to a blackjack server
pub struct Connection {
    /// The handle to the socket
    handle: SOCKET,
}
