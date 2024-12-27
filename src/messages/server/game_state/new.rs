use crate::{
    messages::{GameStateServerMessage, RefCow, ServerMessage},
    model::BlackjackTable,
};

impl<'a> GameStateServerMessage<'a> {
    /// Create a new [`GameStateServerMessage`]
    pub fn new<T: Into<RefCow<'a, BlackjackTable>>>(client_id: u8, table: T) -> ServerMessage<'a> {
        ServerMessage::GameState(GameStateServerMessage {
            client_id,
            table: table.into(),
        })
    }
}
