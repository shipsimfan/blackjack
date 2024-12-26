use crate::messages::HelloServerMessage;
use std::borrow::Cow;

impl<'a> HelloServerMessage<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> HelloServerMessage<'static> {
        HelloServerMessage {
            protocol_version: self.protocol_version,
            password_required: self.password_required,
            server_name: match self.server_name {
                Cow::Borrowed(str) => Cow::Owned(str.to_owned()),
                Cow::Owned(str) => Cow::Owned(str),
            },
            server_application_name: match self.server_application_name {
                Cow::Borrowed(str) => Cow::Owned(str.to_owned()),
                Cow::Owned(str) => Cow::Owned(str),
            },
            server_version: self.server_version,
        }
    }
}
