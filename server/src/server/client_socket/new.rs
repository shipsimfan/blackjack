use super::{ClientSocket, ClientWriter, ReadState, HEADER_SIZE};
use crate::server::{epoll::EPoll, Handle};
use linux::sys::epoll::EPOLLIN;
use std::{cell::RefCell, rc::Rc};

impl ClientSocket {
    /// Create a new client socket
    pub fn new(
        mut handle: Handle,
        epoll: Rc<RefCell<EPoll>>,
        id: u64,
        clients_to_disconnect: Rc<RefCell<Vec<usize>>>,
    ) -> linux::Result<(Self, ClientWriter)> {
        handle.register(epoll, id, EPOLLIN)?;
        let handle = Rc::new(RefCell::new(Some(handle)));

        Ok((
            ClientSocket {
                handle: handle.clone(),
                read_state: ReadState::Header,
                header_buffer: [0; HEADER_SIZE],
                body_buffer: Vec::with_capacity(u16::MAX as usize),
                last_tag: 0,
                read_length: 0,
                id: id as usize,
                clients_to_disconnect: clients_to_disconnect.clone(),
            },
            ClientWriter::new(handle, id as _, clients_to_disconnect),
        ))
    }
}
