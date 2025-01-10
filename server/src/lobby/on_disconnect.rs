use blackjack::messages::ClientDisconnectedServerMessage;

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

        let client = self.clients[client_id].take();
        if client.is_some() {
            println!(
                "[INFO] {} (client {}) disconnected",
                self.table.player(client_id).username(),
                client_id
            );

            let message = ClientDisconnectedServerMessage::new(client_id);
            self.handle_server_message(message);
        }
    }
}
