use super::Lobby;
use blackjack::{
    messages::{ClientMessage, ErrorServerMessage, GameStateServerMessage, ServerMessage},
    model::Player,
};

impl Lobby {
    /// Called when a message is received by the server
    pub fn on_message(&mut self, client: usize, message: ClientMessage) {
        println!("[INFO] Message {} from client {}", message.tag(), client);

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

        self.table.add_player(player, client_id);

        todo!("send client connect message to all connected clients");

        client.send(&ServerMessage::GameState(GameStateServerMessage::Borrowed(
            &self.table,
        )));
        self.clients[client_id] = Some(client.unwrap());
    }
}
