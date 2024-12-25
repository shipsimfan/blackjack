use crate::messages::{ErrorServerMessage, ServerMessage};

impl ErrorServerMessage {
    /// Creates a new [`ErrorServerMessage::ServerFull`]
    pub fn server_full<'a>() -> ServerMessage<'a> {
        ServerMessage::Error(ErrorServerMessage::ServerFull)
    }

    /// Creates a new [`ErrorServerMessage::ConnectionTimeout`]
    pub fn connection_timeout<'a>() -> ServerMessage<'a> {
        ServerMessage::Error(ErrorServerMessage::ConnectionTimeout)
    }
}
