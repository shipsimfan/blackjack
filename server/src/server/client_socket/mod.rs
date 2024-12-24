use super::Handle;
use blackjack::messages::header::HEADER_SIZE;
use read_state::ReadState;
use std::{cell::RefCell, rc::Rc};

mod disconnect;
mod error;
mod read_state;
mod writer;

mod new;
mod read;

pub use error::ReadMessageError;
pub use writer::ClientWriter;

/// A socket representing a client
pub(super) struct ClientSocket {
    /// The handle to the client socket
    handle: Rc<RefCell<Option<Handle>>>,

    /// The current item being read
    read_state: ReadState,

    /// The buffer for the header of the message
    header_buffer: [u8; HEADER_SIZE],

    /// The buffer for the body of the message
    body_buffer: Vec<u8>,

    /// The tag in the last read header
    last_tag: u16,

    /// The currently read size of the header or body
    read_length: usize,

    /// The id of this client
    id: usize,

    /// The list of clients to disconnect
    clients_to_disconnect: Rc<RefCell<Vec<usize>>>,
}
