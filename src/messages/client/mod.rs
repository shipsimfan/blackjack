use crate::messages::messages;

mod chat;
mod hello;
mod hit;
mod place_bet;
mod play_next_round;
mod stand;

pub use chat::ChatClientMessage;
pub use hello::{HelloClientMessage, Username};
pub use hit::HitClientMessage;
pub use place_bet::PlaceBetClientMessage;
pub use play_next_round::PlayNextRoundClientMessage;
pub use stand::StandClientMessage;

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

        /// A player has tried placing a bet
        PlaceBet(PlaceBetClientMessage) = 4,

        /// A player wants to hit on their hand
        Hit(HitClientMessage) = 5,

        /// A player wants to stand on their hand
        Stand(StandClientMessage) = 6,
    }
);
