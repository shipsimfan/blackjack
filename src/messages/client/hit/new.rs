use crate::messages::{ClientMessage, HitClientMessage};

impl HitClientMessage {
    /// Creates a new [`HitClientMessage`]
    pub fn new<'a>() -> ClientMessage<'a> {
        ClientMessage::Hit(HitClientMessage {})
    }
}
