use crate::{client::Move, messages::ServerMessage, model::BlackjackTable};
use std::num::NonZeroU16;

/// An AI that can play blackjack
pub trait AI {
    /// Called when the clients receives any message
    #[allow(unused_variables)]
    fn on_message(&mut self, message: &ServerMessage, table: &BlackjackTable) {}

    /// A request for the client to make a move
    fn make_move(&mut self, table: &BlackjackTable) -> Move;

    /// Request for the client to place a bet
    fn place_bet(&mut self, table: &BlackjackTable) -> NonZeroU16;
}
