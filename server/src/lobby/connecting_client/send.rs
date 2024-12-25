use super::ConnectingClient;
use blackjack::messages::ServerMessage;

impl ConnectingClient {
    /// Send `message` to this client
    pub fn send(&mut self, message: &ServerMessage) {
        self.writer.send(message);
    }
}
