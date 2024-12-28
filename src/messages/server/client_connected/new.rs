use crate::{
    messages::{ClientConnectedServerMessage, RefCow, ServerMessage},
    model::Player,
};

impl<'a> ClientConnectedServerMessage<'a> {
    /// Creates a new [`ClientConnectedServerMessage`]
    pub fn new<P: Into<RefCow<'a, Player>>>(player: P, id: u8) -> ServerMessage<'a> {
        ServerMessage::ClientConnected(ClientConnectedServerMessage {
            player: player.into(),
            id,
        })
    }
}
