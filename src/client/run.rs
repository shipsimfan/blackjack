use crate::client::{Client, ClientError, AI};

impl<T: AI> Client<T> {
    /// Run the client
    pub(super) fn run(mut self) -> Result<(), ClientError<T::CreationError>> {
        while let Some(message) = self.socket.read_message()? {}

        println!("Disconnected by server");
        Ok(())
    }
}
