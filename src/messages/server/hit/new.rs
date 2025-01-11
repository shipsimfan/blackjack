use crate::{
    messages::{HitServerMessage, ServerMessage},
    model::Card,
};

impl HitServerMessage {
    /// Creates a new [`HitServerMessage`]
    pub fn new<'a>(card: Card) -> ServerMessage<'a> {
        ServerMessage::Hit(HitServerMessage { card })
    }
}
