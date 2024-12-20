use crate::server::handle::Handle;
use std::{cell::RefCell, rc::Rc};

mod new;

/// A connection to a client that can send messages
pub struct ClientWriter {
    /// The handle to send the messages over
    handle: Rc<RefCell<Option<Handle>>>,

    /// The id of this client
    id: usize,

    /// The list of clients to disconnect
    clients_to_disconnect: Option<Rc<RefCell<Vec<usize>>>>,
}
