use crate::messages::ClientConnectedServerMessage;

impl<'a> ClientConnectedServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> ClientConnectedServerMessage<'static> {
        match self {
            ClientConnectedServerMessage::Borrowed(borrowed) => {
                ClientConnectedServerMessage::Owned(borrowed.clone())
            }
            ClientConnectedServerMessage::Owned(owned) => {
                ClientConnectedServerMessage::Owned(owned)
            }
        }
    }
}
