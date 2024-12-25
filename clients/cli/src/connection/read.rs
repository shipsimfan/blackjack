use super::ReadState;
use crate::Connection;
use blackjack::messages::{header, ServerMessage};
use win32::{try_get_last_error, GetOverlappedResult, FALSE};

impl Connection {
    /// Read a message from this connection
    pub fn read(
        &mut self,
    ) -> Result<Option<Option<ServerMessage<'static>>>, Box<dyn std::error::Error>> {
        let mut bytes_read = 0;
        try_get_last_error!(GetOverlappedResult(
            self.handle as _,
            self.overlapped.as_mut(),
            &mut bytes_read,
            FALSE
        ))?;

        if bytes_read == 0 {
            return Ok(Some(None));
        }

        self.read_length += bytes_read as usize;

        let result = match self.read_state {
            ReadState::Header => self.read_header().map(|_| None),
            ReadState::Body => self.read_body(),
        }?;

        self.begin_read()?;

        Ok(result)
    }

    /// Read a header from the connection
    fn read_header(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.read_length < self.header_buffer.len() {
            return Ok(());
        }

        let (tag, body_length) = header::validate(&self.header_buffer, true).unwrap();

        self.last_tag = tag;
        unsafe { self.body_buffer.set_len(body_length as _) };
        self.read_length = 0;
        self.read_state = ReadState::Body;
        Ok(())
    }

    /// Read a body from the connection
    fn read_body(
        &mut self,
    ) -> Result<Option<Option<ServerMessage<'static>>>, Box<dyn std::error::Error>> {
        if self.read_length < self.body_buffer.len() {
            return Ok(None);
        }

        self.read_length = 0;
        self.read_state = ReadState::Header;

        let message = ServerMessage::parse(self.last_tag, &self.body_buffer)?;
        Ok(Some(Some(message)))
    }
}
