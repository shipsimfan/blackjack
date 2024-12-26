use crate::messages::GameStateServerMessage;

impl<'a> GameStateServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> GameStateServerMessage<'static> {
        match self {
            GameStateServerMessage::Borrowed(borrowed) => {
                GameStateServerMessage::Owned(borrowed.clone())
            }
            GameStateServerMessage::Owned(owned) => GameStateServerMessage::Owned(owned),
        }
    }
}
