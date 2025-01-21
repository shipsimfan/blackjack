use crate::client::{Client, AI};

impl<T: AI> Client<T> {
    /// Creates a new [`Client`]
    pub fn new(ai: T) -> Self {
        Client { ai }
    }
}
