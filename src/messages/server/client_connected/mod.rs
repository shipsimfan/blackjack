use crate::{messages::RefCow, model::Player};

mod generate;
mod new;
mod parse;
mod to_static;
mod unwrap;

/// A client has connected to the server
#[derive(Debug, Clone)]
pub struct ClientConnectedServerMessage<'a> {
    /// The player that has connected
    pub player: RefCow<'a, Player>,
}
