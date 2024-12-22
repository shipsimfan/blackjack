use crate::messages::messages;

mod error;
mod hello;

pub use error::ErrorServerMessage;
pub use hello::HelloServerMessage;

messages!(
    /// The set of messages that can be sent from the server to the client
    #[derive(Debug, Clone)]
    [client = false]
    pub enum ServerMessage {
        /// An error being reported from the server, likely to lead to disconnection
        Error(ErrorServerMessage) = 0,

        /// The initial hello message giving the client information about the server
        Hello(HelloServerMessage) = 1,
    }
);
