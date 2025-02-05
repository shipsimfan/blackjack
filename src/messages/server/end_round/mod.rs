use crate::model::Card;

mod generate;
mod new;
mod parse;

/// The round has ended
#[derive(Debug, Clone)]
pub struct EndRoundServerMessage {
    /// The hidden card of the dealer, if one needs to be revealed
    pub dealer_card: Option<Card>,

    /// The cards the dealer will add to their hand in final play
    pub dealer_play: Vec<Card>,
}
