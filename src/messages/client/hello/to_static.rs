use crate::messages::HelloClientMessage;
use std::borrow::Cow;

impl<'a> HelloClientMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> HelloClientMessage<'static> {
        HelloClientMessage {
            username: self.username.to_static(),
            password: match self.password {
                Cow::Borrowed(str) => Cow::Owned(str.to_owned()),
                Cow::Owned(str) => Cow::Owned(str),
            },
            client_name: match self.client_name {
                Cow::Borrowed(str) => Cow::Owned(str.to_owned()),
                Cow::Owned(str) => Cow::Owned(str),
            },
            client_version: self.client_version,
            ai: self.ai,
        }
    }
}
