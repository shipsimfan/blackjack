use super::{ClientSocket, ReadMessageError, ReadState};
use blackjack::messages::{header, ClientMessage, DisconnectClientMessage};
use linux::try_linux;
use std::ffi::c_int;

/// Read bytes from a socket
fn read_socket(handle: c_int, buffer: &mut [u8]) -> linux::Result<usize> {
    try_linux!(linux::unistd::read(
        handle,
        buffer.as_mut_ptr().cast(),
        buffer.len() as _
    ))
    .map(|bytes| bytes as usize)
}

impl ClientSocket {
    /// Continues reading the next message from this socket
    pub fn read(&mut self) -> Option<ClientMessage> {
        let handle_ref = self.handle.borrow();
        let handle = match &*handle_ref {
            Some(handle) => **handle,
            None => return None,
        };
        drop(handle_ref);

        let result = match self.read_state {
            ReadState::Header => self.read_header(handle),
            ReadState::Body => self.read_body(true, handle),
        };

        match result {
            Ok(result) => result,
            Err(error) => {
                eprintln!("[ERROR] Error while reading from client: {}", error);
                *self.handle.borrow_mut() = None;
                self.clients_to_disconnect.borrow_mut().push(self.id);
                None
            }
        }
    }

    /// Continue reading the header
    fn read_header(&mut self, handle: c_int) -> Result<Option<ClientMessage>, ReadMessageError> {
        let mut first = true;
        while self.read_length < self.header_buffer.len() {
            let bytes = read_socket(handle, &mut self.header_buffer[self.read_length..])?;
            if bytes == 0 {
                if first {
                    return Ok(Some(ClientMessage::Disconnect(DisconnectClientMessage)));
                }

                break;
            }

            first = false;
            self.read_length += bytes;
        }

        if self.read_length < self.header_buffer.len() {
            return Ok(None);
        }

        let (tag, body_length) = header::validate(&self.header_buffer, false)?;

        self.last_tag = tag;
        unsafe { self.body_buffer.set_len(body_length as _) };
        self.read_length = 0;
        self.read_state = ReadState::Body;
        return self.read_body(false, handle);
    }

    /// Continue reading the body
    fn read_body(
        &mut self,
        disconnect_first: bool,
        handle: c_int,
    ) -> Result<Option<ClientMessage>, ReadMessageError> {
        let mut first = true;
        while self.read_length < self.body_buffer.len() {
            let bytes = read_socket(handle, &mut self.body_buffer[self.read_length..])?;
            if bytes == 0 {
                if first && disconnect_first {
                    return Ok(Some(ClientMessage::Disconnect(DisconnectClientMessage)));
                }

                break;
            }

            first = false;
            self.read_length += bytes;
        }

        if self.read_length < self.body_buffer.len() {
            return Ok(None);
        }

        self.read_length = 0;
        self.read_state = ReadState::Header;

        ClientMessage::parse(self.last_tag, &self.body_buffer)
            .map(Some)
            .map_err(Into::into)
    }
}
