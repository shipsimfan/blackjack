use super::Lobby;
use blackjack::messages::ErrorServerMessage;
use std::time::SystemTime;

impl Lobby {
    /// Called when a wait for events times out
    pub fn on_timeout(&mut self) {
        let now = SystemTime::now();

        let mut last_disconnected = 0;
        for i in 0..self.connecting_clients.len() {
            if self.connecting_clients[i].timeout() > now {
                last_disconnected = i;
                break;
            }

            self.connecting_clients[i].send(&ErrorServerMessage::connection_timeout());
            self.connecting_clients[i].disconnect();

            println!("[INFO] Rejecting client due to connection timeout")
        }

        for _ in 0..last_disconnected {
            self.connecting_clients.pop_front();
        }
    }
}
