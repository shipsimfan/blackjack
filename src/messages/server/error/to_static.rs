use crate::messages::ErrorServerMessage;

impl ErrorServerMessage {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> Self {
        self
    }
}
