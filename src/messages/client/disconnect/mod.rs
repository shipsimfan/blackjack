mod generate;
mod parse;

/// A client has disconnected from the server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisconnectClientMessage;
