use crate::Lobby;
use blackjack::messages::ServerMessage;

impl Lobby {
    /// Send `message` to all clients
    pub fn send_all(&mut self, message: &ServerMessage) {
        message.generate(&mut self.write_all_buffer);

        for client in &mut self.clients {
            if let Some(client) = client {
                unsafe { client.send_raw(&self.write_all_buffer) };
            }
        }
    }
}
