use crate::messages::GameStateServerMessage;

impl<'a> GameStateServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> GameStateServerMessage<'static> {
        GameStateServerMessage {
            client_id: self.client_id,
            table: self.table.to_static(),
        }
    }
}
