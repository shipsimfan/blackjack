use blackjack::messages::ErrorServerMessage;

use super::Lobby;
use crate::server::ClientWriter;

impl Lobby {
    /// Called when a client connects but the server is full
    pub fn on_server_full(&self, mut client: ClientWriter) {
        println!("[INFO] Rejected a connecting client due to server being full");
        client.send(&ErrorServerMessage::server_full());
    }
}
