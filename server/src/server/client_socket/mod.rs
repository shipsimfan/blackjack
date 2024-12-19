use super::Handle;
use read_state::ReadState;

mod read_state;

mod new;

/// A socket representing a client
pub(super) struct ClientSocket {
    /// The handle to the client socket
    handle: Handle,

    /// The current item being read
    read_state: ReadState,

    /// The buffer for the header of the message
    header_buffer: [u8; HEADER_SIZE],

    /// The buffer for the body of the message
    body_buffer: Vec<u8>,

    /// The tag in the last read header
    last_tag: u16,

    /// The size in the last read header
    last_body_size: usize,
}

const MAGIC: [u8; 4] = *b"BKJK";
const TAG_OFFSET: usize = MAGIC.len();
const LEN_OFFSET: usize = TAG_OFFSET + 2;
const HEADER_SIZE: usize = LEN_OFFSET + 2;
