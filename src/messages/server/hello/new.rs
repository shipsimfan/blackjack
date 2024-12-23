use crate::messages::{HelloServerMessage, ServerMessage, Version, CURRENT_PROTOCOL_VERSION};

impl HelloServerMessage {
    /// Creates a new [`HelloServerMessage`]
    pub fn new(
        password_required: bool,
        server_name: String,
        server_application_name: String,
        server_version: Version,
    ) -> ServerMessage {
        ServerMessage::Hello(HelloServerMessage {
            protocol_version: CURRENT_PROTOCOL_VERSION,
            password_required,
            server_name,
            server_application_name,
            server_version,
        })
    }
}
