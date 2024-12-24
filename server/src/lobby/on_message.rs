use super::Lobby;
use blackjack::messages::ClientMessage;

impl Lobby {
    /// Called when a message is received by the server
    pub fn on_message(&mut self, client: usize, message: ClientMessage) {}
}
