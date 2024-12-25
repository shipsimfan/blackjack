use crate::messages::{ErrorServerMessage, ServerMessage};

impl ErrorServerMessage {
    /// Creates a new [`ErrorServerMessage::ServerFull`]
    pub fn server_full<'a>() -> ServerMessage<'a> {
        ServerMessage::Error(ErrorServerMessage::ServerFull)
    }
}
