mod display;
mod generate;
mod new;
mod parse;

/// An error reported by the server to a client
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorServerMessage {
    /// The server is full
    ServerFull = 0,
}

impl std::error::Error for ErrorServerMessage {}
