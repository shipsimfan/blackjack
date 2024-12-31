use crate::model::HandValue;

impl HandValue {
    /// Gets the value as a [`u8`]
    pub fn as_u8(&self) -> u8 {
        *match self {
            HandValue::NoAce(value) => value,
            HandValue::Hard(value) => value,
            HandValue::Soft(value) => value,
        }
    }
}
