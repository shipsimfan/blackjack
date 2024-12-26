use crate::messages::{ClientDisconnectedServerMessage, ServerMessage};

impl ClientDisconnectedServerMessage {
    /// Create a new [`ClientDisconnectedServerMessage`]
    pub fn new<'a>(id: usize) -> ServerMessage<'a> {
        ServerMessage::ClientDisconnected(ClientDisconnectedServerMessage { id: id as u8 })
    }
}
