use super::Lobby;

impl Lobby {
    /// Called when a client disconnects
    pub fn on_disconnect(&mut self, client_id: usize) {
        for i in 0..self.connecting_clients.len() {
            if self.connecting_clients[i].id() == client_id {
                eprintln!("[WARN] A client disconnected before hello message");
                self.connecting_clients.remove(i);
                return;
            }
        }

        todo!("Handle disconnecting a properly connected client");
    }
}
