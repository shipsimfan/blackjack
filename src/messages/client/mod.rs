use crate::messages::messages;

mod disconnect;
mod hello;

pub use disconnect::DisconnectClientMessage;
pub use hello::{HelloClientMessage, Username};

messages!(
    /// The set of messages that can be sent from the client to the server
    pub enum ClientMessage {
        /// The initial hello message giving the server information about the client
        Hello(HelloClientMessage) = 1,

        /// The client has disconnected from the server
        Disconnect(DisconnectClientMessage) = 2,
    }
);
