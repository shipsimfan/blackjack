use crate::messages::Version;

mod generate;
mod parse;

/// The message sent by the server to begin communication with a client
#[derive(Debug, Clone)]
pub struct HelloServerMessage {
    /// The protocol version used by the server
    protocol_version: u32,

    /// Is a password required to enter the server?
    password_required: bool,

    /// The name of this server
    server_name: String,

    /// The name of the server application
    server_application_name: String,

    /// The version of the server application
    server_version: Version,
}
