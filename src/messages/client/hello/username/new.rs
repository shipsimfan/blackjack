use crate::messages::Username;

impl Username {
    /// Creates a new [`Username`], validating the contents of `username`
    pub fn new(username: String) -> Option<Self> {
        if username.len() > u8::MAX as usize {
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
