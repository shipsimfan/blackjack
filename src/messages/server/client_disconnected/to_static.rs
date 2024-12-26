use crate::messages::ClientDisconnectedServerMessage;

impl ClientDisconnectedServerMessage {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> ClientDisconnectedServerMessage {
        self
    }
}
