use crate::messages::Version;
use std::borrow::Cow;

mod generate;
mod get;
mod new;
mod parse;
mod to_static;
mod unwrap;

/// The message sent by the server to begin communication with a client
#[derive(Debug, Clone)]
pub struct HelloServerMessage<'a> {
    /// The protocol version used by the server
    protocol_version: u32,

    /// Is a password required to enter the server?
    password_required: bool,

    /// The name of this server
    server_name: Cow<'a, str>,

    /// The name of the server application
    server_application_name: Cow<'a, str>,

    /// The version of the server application
    server_version: Version,
}
