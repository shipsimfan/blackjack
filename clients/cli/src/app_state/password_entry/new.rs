use super::PasswordEntryState;
use crate::Options;

impl PasswordEntryState {
    pub fn new(options: Options) -> Self {
        PasswordEntryState { options }
    }
}
