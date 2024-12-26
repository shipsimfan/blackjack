use crate::model::Player;

mod generate;
mod parse;
mod to_static;
mod unwrap;

/// A client has connected to the server
#[derive(Debug, Clone)]
pub enum ClientConnectedServerMessage<'a> {
    /// The player is borrowed
    Borrowed(&'a Player),

    /// The player is owned
    Owned(Player),
}
