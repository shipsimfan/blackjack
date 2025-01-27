mod display;
mod generate;
mod new;
mod parse;
mod to_static;

/// An error reported by the server to a client
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorServerMessage {
    /// The server is full
    ServerFull = 0,

    /// The client took too long to respond to the server hello
    ConnectionTimeout = 1,

    /// The client sent an invalid password when connecting
    InvalidPassword = 2,

    /// Another client on the server already has the same username
    UsernameTaken = 3,
}

impl std::error::Error for ErrorServerMessage {}
