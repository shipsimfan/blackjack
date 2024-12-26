use crate::{messages::GameStateServerMessage, model::BlackjackTable};

impl<'a> GameStateServerMessage<'a> {
    /// Unwraps the [`BlackjackTable`] contained into an owned version
    pub fn unwrap(self) -> BlackjackTable {
        match self {
            GameStateServerMessage::Borrowed(borrowed) => borrowed.clone(),
            GameStateServerMessage::Owned(owned) => owned,
        }
    }
}
