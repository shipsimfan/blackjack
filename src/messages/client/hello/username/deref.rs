use crate::messages::Username;
use std::ops::Deref;

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
