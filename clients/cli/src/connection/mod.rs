use blackjack::messages::ClientMessage;
use read_state::ReadState;
use std::collections::VecDeque;
use win32::{winsock2::SOCKET, HANDLE, OVERLAPPED};

mod begin_read;
mod connect;
mod drop;
mod get;
mod read;
mod read_state;
mod send;

pub use connect::ConnectionError;

/// A connection to a blackjack server
pub struct Connection {
    /// The handle to the socket
    handle: SOCKET,

    /// The handle to the event that signals on successful read
    read_event: HANDLE,

    /// The overlapped struct monitoring the current read operation
    read_overlapped: Box<OVERLAPPED>,

    /// The current item being read
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
