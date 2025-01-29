//! A framework for implementing AI clients

use std::net::TcpStream;

mod ai;
mod error;
mod r#move;
mod new;
mod options;
mod run;

pub use ai::AI;
pub use error::ClientError;
pub use options::Options;
pub use r#move::Move;

/// A framework for implementing AI clients
pub struct Client<T: AI> {
    /// The ai running this client
    ai: T,

    /// The socket connected to the server
    socket: TcpStream,
}
