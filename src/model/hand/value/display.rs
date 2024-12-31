use crate::model::HandValue;

impl std::fmt::Display for HandValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandValue::NoAce(value) => value.fmt(f),
            HandValue::Hard(value) => write!(f, "H{}", value),
            HandValue::Soft(value) => write!(f, "S{}", value),
        }
    }
}
