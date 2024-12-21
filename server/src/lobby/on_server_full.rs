use super::Lobby;
use crate::server::ClientWriter;

impl Lobby {
    /// Called when a client connects but the server is full
    pub fn on_server_full(&self, client: ClientWriter) {
        todo!("Send server full message to client");
    }
}
