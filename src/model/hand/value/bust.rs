use crate::model::HandValue;

impl HandValue {
    /// Is this hand a bust?
    pub fn is_bust(&self) -> bool {
        self.as_u8() > 21
    }
}
