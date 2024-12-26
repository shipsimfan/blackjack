mod display;
mod new;

/// An invalid username was passed
#[derive(Debug)]
pub struct InvalidUsernameError {
    /// The username
    username: String,
}

impl std::error::Error for InvalidUsernameError {}
