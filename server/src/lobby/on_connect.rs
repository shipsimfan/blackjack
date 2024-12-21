use super::{connecting_client::ConnectingClient, Lobby};
use crate::server::ClientWriter;

impl Lobby {
    /// Called when a client connects
    pub fn on_connect(&mut self, writer: ClientWriter) {
        self.connecting_clients
            .push_back(ConnectingClient::new(writer, self.connection_timeout));
    }
}
