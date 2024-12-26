use crate::InvalidUsernameError;

impl std::fmt::Display for InvalidUsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid username \"{}\"", self.username)
    }
}
