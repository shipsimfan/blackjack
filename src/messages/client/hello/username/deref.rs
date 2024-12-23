use crate::messages::Username;
use std::ops::Deref;

impl<'a> Deref for Username<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> AsRef<str> for Username<'a> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
