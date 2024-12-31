use crate::model::Hand;
use std::num::NonZeroU16;

impl Hand {
    /// Creates a new empty [`Hand`]
    pub fn new(bet: Option<NonZeroU16>) -> Self {
        Hand { bet }
    }
}
