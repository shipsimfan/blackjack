use super::Connecting;
use crate::{options::InvalidUsernameError, Options};

impl Connecting {
    /// Creates a new [`Connecting`] game state
    pub fn new(options: Options) -> Result<Connecting, InvalidUsernameError> {
        Ok(Connecting {
            username: options.username()?,
        })
    }
}
