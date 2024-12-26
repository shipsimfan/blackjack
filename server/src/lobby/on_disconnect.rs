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

        if let Some(client) = &mut self.clients[client_id] {
            self.table.remove_player(client_id);
            todo!("Send disconnect message to all clients");
        }

        self.clients[client_id] = None;
    }
}
