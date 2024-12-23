use super::{connecting_client::ConnectingClient, Lobby};
use crate::server::ClientWriter;
use blackjack::messages::HelloServerMessage;

impl Lobby {
    /// Called when a client connects
    pub fn on_connect(&mut self, mut writer: ClientWriter) {
        println!("[INFO] Client connecting on slot {}", writer.id());

        writer.send(&HelloServerMessage::new(
            self.password.is_some(),
            &self.server_name,
            "LBlackjack",
            self.server_version,
        ));

        self.connecting_clients
            .push_back(ConnectingClient::new(writer, self.connection_timeout));
    }
}
