use crate::model::Card;

mod generate;
mod new;
mod parse;

/// A round has begun and cards have been dealt
#[derive(Debug, Clone)]
pub struct DealServerMessage {
    /// The dealer's face up card
    pub dealer_up_card: Card,

    /// The dealer's face down card
    ///
    /// This value is [`None`] if the card should start hidden.
    /// This value is [`Some`] if the card should be shown.
    pub dealer_down_card: Option<Card>,

    /// The initial hands for the players, flattened
    pub hands: Vec<(Card, Card)>,
}
