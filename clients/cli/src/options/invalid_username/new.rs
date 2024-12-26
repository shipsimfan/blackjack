use crate::InvalidUsernameError;

impl InvalidUsernameError {
    /// Creates a new [`InvalidUsernameError`]
    pub fn new(username: String) -> Self {
        InvalidUsernameError { username }
    }
}
