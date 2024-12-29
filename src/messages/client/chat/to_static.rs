use crate::messages::ChatClientMessage;
use std::borrow::Cow;

impl<'a> ChatClientMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> ChatClientMessage<'static> {
        ChatClientMessage {
            message: Cow::Owned(match self.message {
                Cow::Borrowed(borrowed) => borrowed.to_owned(),
                Cow::Owned(owned) => owned,
            }),
        }
    }
}
