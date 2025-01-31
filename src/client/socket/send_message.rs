use std::io::Write;

use crate::{
    client::{ClientError, Socket},
    messages::ClientMessage,
};

impl Socket {
    /// Send `message` to the server
    pub fn send_message<'a, T: Into<ClientMessage<'a>>, E: std::error::Error>(
        &mut self,
        message: T,
    ) -> Result<(), ClientError<E>> {
        let message = message.into();
        message.generate(&mut self.write_buffer);
        self.socket
            .write_all(&self.write_buffer)
            .map_err(ClientError::WriteIOError)
    }
}
