use crate::messages::ChatServerMessage;
use std::borrow::Cow;

impl<'a> ChatServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> ChatServerMessage<'static> {
        ChatServerMessage {
            client: self.client,
            message: Cow::Owned(match self.message {
                Cow::Borrowed(borrowed) => borrowed.to_owned(),
                Cow::Owned(owned) => owned,
            }),
        }
    }
}
