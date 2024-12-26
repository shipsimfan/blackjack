use crate::messages::messages;

mod error;
mod game_state;
mod hello;

pub use error::ErrorServerMessage;
pub use game_state::GameStateServerMessage;
pub use hello::HelloServerMessage;

messages!(
    /// The set of messages that can be sent from the server to the client
    #[derive(Debug, Clone)]
    [client = false]
    pub enum ServerMessage<'a> {
        /// An error being reported from the server, likely to lead to disconnection
        Error(ErrorServerMessage) = 0,

        /// The initial hello message giving the client information about the server
        Hello(HelloServerMessage<'a>) = 1,

        /// The current state of the game
        GameState(GameStateServerMessage<'a>) = 2,
    }
);
