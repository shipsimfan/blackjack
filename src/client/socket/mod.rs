use read_state::ReadState;
use std::{io::Read, net::TcpStream};

mod connect;
mod read_state;

/// A connection to a server
pub(super) struct Socket {
    /// The os representation of the connection
    socket: TcpStream,

    /// The current part of the message being read
    read_state: ReadState,

    /// The buffer for the header of the message
    header_buffer: Box<[u8]>,

    /// The buffer for the body of the message
    body_buffer: Vec<u8>,

    /// The tag in the last read header
    last_tag: u16,

    /// The currently read size of the header or body
    read_length: usize,

    /// The buffer for putting packets to be written
    write_buffer: Vec<u8>,
}
