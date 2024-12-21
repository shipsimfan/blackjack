use super::ClientWriter;

impl ClientWriter {
    /// Disconnects this client
    pub fn disconnect(&mut self) {
        *self.handle.borrow_mut() = None;
        match &self.clients_to_disconnect {
            Some(clients_to_disconnect) => clients_to_disconnect.borrow_mut().push(self.id),
            None => {}
        }
    }
}
