use crate::messages::Username;
use std::borrow::Cow;

impl<'a> Username<'a> {
    /// Creates a new [`Username`], validating the contents of `username`
    pub fn new<S: Into<Cow<'a, str>>>(username: S) -> Option<Self> {
        let username = username.into();

        if username.len() > u8::MAX as usize || username.len() == 0 {
            return None;
        }

        for c in username.chars() {
            if (!c.is_alphanumeric() && !c.is_ascii_graphic()) || c == '[' || c == ']' {
                return None;
            }
        }

        Some(Username { inner: username })
    }
}
