use super::Lobby;
use blackjack::{
    messages::{
        ChatServerMessage, ClientConnectedServerMessage, ClientMessage, ErrorServerMessage,
        GameStateServerMessage, ServerMessage,
    },
    model::{HandleMessageResult, Player},
};

impl Lobby {
    /// Called when a message is received by the server
    pub fn on_message(&mut self, client_id: usize, message: ClientMessage) {
        if self.clients[client_id].is_none() {
            return self.on_connecting_client_message(client_id, message);
        }

        if let Some((server_message, shuffle)) = self.table.translate_message(client_id, &message) {
            if let Some(shuffle) = shuffle {
                self.send_all(&shuffle);
            }

            self.handle_server_message(server_message);
            return;
        }

        match message {
            ClientMessage::Chat(chat) => {
                eprintln!(
                    "[CHAT] [{}] {}",
                    self.table.player(client_id).username(),
                    chat.message
                );
                let chat = ChatServerMessage::new(client_id as _, chat.message);
                self.send_all(&chat);
            }

            _ => {
                eprintln!(
                    "[ERROR] Unexpected message from client {} - {}",
                    client_id,
                    message.tag()
                );
                self.clients[client_id].as_mut().unwrap().disconnect();
            }
        }
    }

    /// Handles a server message, returning `true` if the model was changed
    pub(super) fn handle_server_message(&mut self, message: ServerMessage) {
        let mut server_message = Some(message);
        while let Some(message) = server_message.take() {
            self.send_all(&message);
            match self.table.handle_message(message) {
                HandleMessageResult::Deal(deal, shuffle) => {
                    if let Some(shuffle) = shuffle {
                        self.send_all(&shuffle);
                    }
                    server_message = Some(deal);
                }
                HandleMessageResult::EndRound(end_round, shuffle) => {
                    if let Some(shuffle) = shuffle {
                        self.send_all(&shuffle);
                    }
                    server_message = Some(end_round);
                }
                _ => return,
            }
        }
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

                for player in self.table.sitting_players() {
                    if *player.username() == hello.username {
                        eprintln!("[WARN] Client tried connecting with a taken username");
                        client.send(&ErrorServerMessage::username_taken());
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
