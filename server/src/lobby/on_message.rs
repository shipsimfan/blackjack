use super::Lobby;
use blackjack::{
    messages::{
        ClientConnectedServerMessage, ClientMessage, ErrorServerMessage, GameStateServerMessage,
    },
    model::Player,
};

impl Lobby {
    /// Called when a message is received by the server
    pub fn on_message(&mut self, client: usize, message: ClientMessage) {
        if self.clients[client].is_none() {
            return self.on_connecting_client_message(client, message);
        }

        todo!("Handle message from regular client");
    }

    /// Handle a message from a currently connecting client
    fn on_connecting_client_message(&mut self, client_id: usize, message: ClientMessage) {
        let mut index = None;
        for i in 0..self.connecting_clients.len() {
            if self.connecting_clients[i].id() == client_id {
                index = Some(i);
                break;
            }
        }

        let mut client = self.connecting_clients.remove(index.unwrap()).unwrap();

        let player = match message {
            ClientMessage::Hello(hello) => {
                if let Some(password) = self.password.as_ref() {
                    if hello.password.is_none() || hello.password.as_ref().unwrap() != password {
                        eprintln!("[WARN] Client tried connecting with invalid password");
                        client.send(&ErrorServerMessage::invalid_password());
                        client.disconnect();
                        return;
                    }
                }

                Player::new(hello.username.to_static(), hello.ai)
            }
            _ => {
                eprintln!("[WARN] Unexpected message from connecting client");
                client.disconnect();
                return;
            }
        };

        println!(
            "[INFO] {} connected on slot {} (AI: {})",
            player.username(),
            client_id,
            player.ai()
        );

        let message = ClientConnectedServerMessage::new(&player, client_id as _);
        self.send_all(&message);
        self.table.handle_message(message);

        client.send(&GameStateServerMessage::new(client_id as _, &self.table));
        self.clients[client_id] = Some(client.unwrap());
    }
}
