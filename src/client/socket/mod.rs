use std::net::TcpStream;

mod connect;
mod read_message;

/// A connection to a server
pub(super) struct Socket {
    /// The os representation of the connection
    socket: TcpStream,

    /// The buffer for the body of the message
    body_buffer: Vec<u8>,

    /// The buffer for putting packets to be written
    write_buffer: Vec<u8>,
}
