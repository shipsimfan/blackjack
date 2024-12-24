use crate::messages::messages;

mod hello;

pub use hello::{HelloClientMessage, Username};

messages!(
    /// The set of messages that can be sent from the client to the server
    #[derive(Debug, Clone)]
    [client = true]
    pub enum ClientMessage<'a> {
        /// The initial hello message giving the server information about the client
        Hello(HelloClientMessage<'a>) = 1,
    }
);
