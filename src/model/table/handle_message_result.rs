use crate::messages::ServerMessage;

/// The result of the model handling a message
#[derive(Debug, Clone)]
pub enum HandleMessageResult<'a> {
    /// No change occured to the model
    NoChange,

    /// A change occured to the model
    Change,

    /// A new hand should be dealt
    Deal(ServerMessage<'a>, Option<ServerMessage<'a>>),

    /// The round should be ended
    EndRound(ServerMessage<'a>, Option<ServerMessage<'a>>),
}

impl<'a> HandleMessageResult<'a> {
    /// Does this result indicate a change to the model?
    pub fn is_change(&self) -> bool {
        match self {
            HandleMessageResult::Change
            | HandleMessageResult::Deal(_, _)
            | HandleMessageResult::EndRound(_, _) => true,
            HandleMessageResult::NoChange => false,
        }
    }
}
