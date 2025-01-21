use std::num::NonZeroU8;

mod apply;
mod display;
mod flag;
mod generate;
mod new;
mod parse;

/// A ratio used for a blackjack payout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio {
    /// The numerator of the ratio
    pub numerator: NonZeroU8,

    /// The denominator of the ratio
    pub denominator: NonZeroU8,
}
