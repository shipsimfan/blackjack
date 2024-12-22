use crate::messages::Version;

mod username;

mod generate;
mod parse;

pub use username::Username;

/// The message sent by the client in response to server hello
#[derive(Debug, Clone)]
pub struct HelloClientMessage {
    /// The username of the client
    pub username: Username,

    /// The password the client gave to try and join the session
    pub password: String,

    /// The name of the client software
    pub client_name: String,

    /// The version of the client software
    pub client_version: Version,

    /// Has the client self-reported as an AI?
    pub ai: bool,
}
