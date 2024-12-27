use crate::{
    messages::{ClientConnectedServerMessage, RefCow, ServerMessage},
    model::Player,
};

impl<'a> ClientConnectedServerMessage<'a> {
    /// Creates a new [`ClientConnectedServerMessage`]
    pub fn new<P: Into<RefCow<'a, Player>>>(player: P) -> ServerMessage<'a> {
        ServerMessage::ClientConnected(ClientConnectedServerMessage {
            player: player.into(),
        })
    }
}
