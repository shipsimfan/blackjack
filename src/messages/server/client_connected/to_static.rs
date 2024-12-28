use crate::messages::ClientConnectedServerMessage;

impl<'a> ClientConnectedServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> ClientConnectedServerMessage<'static> {
        ClientConnectedServerMessage {
            player: self.player.to_static(),
            id: self.id,
        }
    }
}
