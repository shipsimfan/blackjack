use crate::client::{Client, AI};

impl<T: AI> Client<T> {
    /// Run the client
    pub fn run(self) -> Result<(), std::io::Error> {
        loop {}
    }
}
