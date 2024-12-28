use std::borrow::Cow;

use crate::messages::HelloServerMessage;

impl<'a> HelloServerMessage<'a> {
    /// Unwraps the server name from this message
    pub fn unwrap_server_name(self) -> String {
        match self.server_name {
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
            Cow::Owned(owned) => owned,
        }
    }
}
