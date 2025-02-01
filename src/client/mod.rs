//! A framework for implementing AI clients

use socket::Socket;

mod ai;
mod error;
mod r#move;
mod new;
mod options;
mod run;
mod socket;

pub use ai::AI;
pub use error::ClientError;
pub use options::Options;
pub use r#move::Move;

use crate::model::BlackjackTable;

/// A framework for implementing AI clients
pub struct Client<T: AI> {
    /// The ai running this client
    ai: T,

    /// The socket connected to the server
    socket: Socket,

    /// The model of the game
    table: BlackjackTable,

    /// The id assigned to the player
    client_id: u8,
}
