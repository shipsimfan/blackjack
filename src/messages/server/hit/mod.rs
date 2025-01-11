use crate::model::Card;

mod generate;
mod new;
mod parse;

/// The current player has hit on their hand
#[derive(Debug, Clone)]
pub struct HitServerMessage {
    /// The card to be added to the player's hand
    pub card: Card,
}
