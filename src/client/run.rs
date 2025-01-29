use crate::client::{Client, ClientError, AI};

impl<T: AI> Client<T> {
    /// Run the client
    pub(super) fn run(self) -> Result<(), ClientError<T::CreationError>> {
        loop {}
    }
}
