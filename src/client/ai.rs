use crate::{
    client::{Move, Options},
    messages::ServerMessage,
    model::BlackjackTable,
};
use std::num::NonZeroU16;

/// An AI that can play blackjack
pub trait AI: Sized {
    /// The options to be passed to this [`AI`] on startup
    type Options: Options;

    /// An error that can occur while creating the AI
    type CreationError: std::error::Error = std::convert::Infallible;

    /// Create a new version of this [`AI`]
    fn new(options: Self::Options, client_id: usize) -> Result<Self, Self::CreationError>;

    /// Called when the clients receives any message
    #[allow(unused_variables)]
    fn on_message(&mut self, message: &ServerMessage, table: &BlackjackTable) {}

    /// A request for the client to make a move
    fn make_move(&mut self, table: &BlackjackTable) -> Move;

    /// Request for the client to place a bet
    fn place_bet(&mut self, table: &BlackjackTable) -> NonZeroU16;
}
