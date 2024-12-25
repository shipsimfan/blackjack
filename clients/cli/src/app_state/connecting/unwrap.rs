use super::Connecting;
use crate::Options;

impl Connecting {
    pub fn unwrap(self) -> Options {
        self.options
    }
}
