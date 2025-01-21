use crate::model::Ratio;
use std::num::NonZeroU8;

impl Ratio {
    /// Creates a new [`Ratio`]
    pub const fn new(numerator: NonZeroU8, denominator: NonZeroU8) -> Self {
        Ratio {
            numerator,
            denominator,
        }
    }

    /// Creates a new [`Ratio`] without checking the values
    pub const unsafe fn new_unchecked(numerator: u8, denominator: u8) -> Self {
        Ratio::new(
            NonZeroU8::new_unchecked(numerator),
            NonZeroU8::new_unchecked(denominator),
        )
    }
}
