use crate::messages::Username;
use std::borrow::Cow;

impl<'a> Username<'a> {
    /// Converts this to a static lifetime
    pub fn to_static(self) -> Username<'static> {
        Username {
            inner: match self.inner {
                Cow::Borrowed(str) => Cow::Owned(str.to_owned()),
                Cow::Owned(str) => Cow::Owned(str),
            },
        }
    }
}
