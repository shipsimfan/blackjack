use super::ClientSocket;

impl ClientSocket {
    pub fn disconnect(&mut self) {
        let mut handle = self.handle.borrow_mut();
        if handle.is_none() {
            return;
        }

        *handle = None;
        self.clients_to_disconnect.borrow_mut().push(self.id);
    }
}
