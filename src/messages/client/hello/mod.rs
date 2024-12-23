use crate::messages::Version;
use std::borrow::Cow;

mod username;

mod generate;
mod parse;

pub use username::Username;

/// The message sent by the client in response to server hello
#[derive(Debug, Clone)]
pub struct HelloClientMessage<'a> {
    /// The username of the client
    pub username: Username<'a>,

    /// The password the client gave to try and join the session
    pub password: Cow<'a, str>,

    /// The name of the client software
    pub client_name: Cow<'a, str>,

    /// The version of the client software
    pub client_version: Version,

    /// Has the client self-reported as an AI?
    pub ai: bool,
}
