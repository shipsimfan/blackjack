use super::{read_state::ReadState, ClientSocket, HEADER_SIZE};
use crate::server::{epoll::EPoll, Handle};
use linux::sys::epoll::EPOLLIN;
use std::{cell::RefCell, rc::Rc};

impl ClientSocket {
    /// Create a new client socket
    pub fn new(mut handle: Handle, epoll: Rc<RefCell<EPoll>>, id: u64) -> linux::Result<Self> {
        handle.register(epoll, id, EPOLLIN)?;

        Ok(ClientSocket {
            handle,
            read_state: ReadState::Header,
            header_buffer: [0; HEADER_SIZE],
            body_buffer: Vec::with_capacity(u16::MAX as usize),
            last_tag: 0,
            last_body_size: 0,
        })
    }
}
