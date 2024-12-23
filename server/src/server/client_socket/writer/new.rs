use super::{ClientWriter, Handle};
use std::{cell::RefCell, rc::Rc};

impl ClientWriter {
    /// Creates a new [`ClientWriter`] for handle
    pub(in crate::server::client_socket) fn new(
        handle: Rc<RefCell<Option<Handle>>>,
        id: usize,
        clients_to_disconnect: Rc<RefCell<Vec<usize>>>,
    ) -> Self {
        ClientWriter {
            handle,
            id,
            clients_to_disconnect: Some(clients_to_disconnect),
            write_buffer: Vec::with_capacity(u16::MAX as usize),
        }
    }

    /// Create a new [`ClientWriter`] for a potentially unregistered handle
    pub(in crate::server) fn new_unregistered(handle: Handle) -> Self {
        ClientWriter {
            handle: Rc::new(RefCell::new(Some(handle))),
            id: 0,
            clients_to_disconnect: None,
            write_buffer: Vec::with_capacity(u16::MAX as usize),
        }
    }
}
