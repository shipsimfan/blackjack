mod as_u8;
mod bust;
mod compute;
mod display;

/// The value a hand can have
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandValue {
    /// The does not contain an ace
    NoAce(u8),

    /// The hand contains an ace and it must take a value of 1
    Hard(u8),

    /// The hand contains an ace and it can take a value of 11
    Soft(u8),
}
