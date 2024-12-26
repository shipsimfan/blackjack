use crate::messages::{ClientMessage, HelloClientMessage, Username, Version};
use std::borrow::Cow;

impl<'a> HelloClientMessage<'a> {
    /// Creates a new [`HelloClientMessage`]
    pub fn new<S: Into<Cow<'a, str>>>(
        username: Username<'a>,
        password: Option<Cow<'a, str>>,
        client_name: S,
        client_version: Version,
        ai: bool,
    ) -> ClientMessage<'a> {
        ClientMessage::Hello(HelloClientMessage {
            username,
            password: password.unwrap_or(Cow::Borrowed("")),
            client_name: client_name.into(),
            client_version,
            ai,
        })
    }
}
