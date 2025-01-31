use crate::{
    client::{ClientError, Socket},
    messages::{
        header::{self, HEADER_SIZE},
        ParseMessageError, ServerMessage,
    },
};
use std::io::Read;

impl Socket {
    /// Reads a message from the socket
    pub fn read_message<'a, E: std::error::Error>(
        &mut self,
    ) -> Result<Option<ServerMessage<'a>>, ClientError<E>> {
        let mut header = [0; HEADER_SIZE];
        let mut i = 0;
        while i < HEADER_SIZE {
            let bytes_read = self
                .socket
                .read(&mut header[i..])
                .map_err(ClientError::ReadIOError)?;
            if bytes_read == 0 {
                break;
            }

            i += bytes_read;
        }

        if i == 0 {
            return Ok(None);
        }

        if i < HEADER_SIZE {
            return Err(ClientError::ParseMessageError(ParseMessageError));
        }

        let (tag, body_length) =
            header::validate(&header, true).map_err(ClientError::ParseMessageError)?;

        unsafe { self.body_buffer.set_len(body_length as usize) };

        self.socket
            .read_exact(&mut self.body_buffer)
            .map_err(ClientError::ReadIOError)?;

        ServerMessage::parse(tag, &self.body_buffer)
            .map(Some)
            .map_err(ClientError::ParseMessageError)
    }
}
