use super::Connecting;
use crate::Options;

impl Connecting {
    /// Creates a new [`Connecting`] game state
    pub fn new(options: Options) -> Connecting {
        Connecting { options }
    }
}
