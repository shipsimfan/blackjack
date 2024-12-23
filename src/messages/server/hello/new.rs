use crate::messages::{HelloServerMessage, ServerMessage, Version, CURRENT_PROTOCOL_VERSION};
use std::borrow::Cow;

impl<'a> HelloServerMessage<'a> {
    /// Creates a new [`HelloServerMessage`]
    pub fn new<S1: Into<Cow<'a, str>>, S2: Into<Cow<'a, str>>>(
        password_required: bool,
        server_name: S1,
        server_application_name: S2,
        server_version: Version,
    ) -> ServerMessage<'a> {
        ServerMessage::Hello(HelloServerMessage {
            protocol_version: CURRENT_PROTOCOL_VERSION,
            password_required,
            server_name: server_name.into(),
            server_application_name: server_application_name.into(),
            server_version,
        })
    }
}
