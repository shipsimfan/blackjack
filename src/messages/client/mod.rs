use crate::messages::messages;

mod chat;
mod hello;
mod play_next_round;

pub use chat::ChatClientMessage;
pub use hello::{HelloClientMessage, Username};
pub use play_next_round::PlayNextRoundClientMessage;

messages!(
    /// The set of messages that can be sent from the client to the server
    #[derive(Debug, Clone)]
    [client = true]
    pub enum ClientMessage<'a> {
        /// The initial hello message giving the server information about the client
        Hello(HelloClientMessage<'a>) = 1,

        /// A chat message sent from the client
        Chat(ChatClientMessage<'a>) = 2,

        /// Notifies if a player will be playing the next round
        PlayNextRound(PlayNextRoundClientMessage) = 3,
    }
);
