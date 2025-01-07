use super::Card;
use std::num::NonZeroUsize;

mod discard;
mod draw;
mod get;
mod new;
mod should_shuffle;
mod shuffle;

/// A shoe of cards that is being played with
#[derive(Debug, Clone)]
pub struct Shoe {
    /// The number of cards in this shoe
    size: usize,

    /// The cards currently in the shoe ready to be played
    cards: Vec<Card>,

    /// When the number of `cards` remaining drops below this, a reshuffle must be done. Set to
    /// [`None`] to indicate a shuffle is needed
    cut_card: Option<NonZeroUsize>,

    /// The discard pile, filled with cards from old hands
    discard: Vec<Card>,
}
