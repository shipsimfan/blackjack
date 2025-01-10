use crate::messages::messages;

mod chat;
mod client_connected;
mod client_disconnected;
mod deal;
mod end_round;
mod error;
mod game_state;
mod hello;
mod place_bet;
mod play_next_round;
mod shuffle;

pub use chat::ChatServerMessage;
pub use client_connected::ClientConnectedServerMessage;
pub use client_disconnected::ClientDisconnectedServerMessage;
pub use deal::DealServerMessage;
pub use end_round::EndRoundServerMessage;
pub use error::ErrorServerMessage;
pub use game_state::GameStateServerMessage;
pub use hello::HelloServerMessage;
pub use place_bet::PlaceBetServerMessage;
pub use play_next_round::PlayNextRoundServerMessage;
pub use shuffle::ShuffleServerMessage;

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

        /// A client connected to the server
        ClientConnected(ClientConnectedServerMessage<'a>) = 3,

        /// A client disconnected from the server
        ClientDisconnected(ClientDisconnectedServerMessage) = 4,

        /// A chat message from a client, echoed by the server
        Chat(ChatServerMessage<'a>) = 5,

        /// A player has changed if they are playing next round
        PlayNextRound(PlayNextRoundServerMessage) = 6,

        /// A player has placed a bet
        PlaceBet(PlaceBetServerMessage) = 7,

        /// A new round has been dealt
        Deal(DealServerMessage) = 8,

        /// The shoe has been shuffled
        Shuffle(ShuffleServerMessage) = 9,

        /// The round has ended
        EndRound(EndRoundServerMessage) = 10,
    }
);
