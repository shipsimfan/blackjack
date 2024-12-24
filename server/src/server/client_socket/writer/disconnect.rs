use super::ClientWriter;

impl ClientWriter {
    /// Disconnects this client
    pub fn disconnect(&mut self) {
        let mut handle = self.handle.borrow_mut();
        if handle.is_none() {
            return;
        }

        *handle = None;
        match &self.clients_to_disconnect {
            Some(clients_to_disconnect) => {
                clients_to_disconnect.borrow_mut().push(self.id);
                self.clients_to_disconnect = None
            }
            None => {}
        }
    }
}
