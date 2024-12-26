use super::ReadState;
use crate::Connection;
use std::ptr::null_mut;
use win32::{GetLastError, ReadFile, ERROR_IO_PENDING, FALSE};

impl Connection {
    /// Begin an asynchronous read operation
    pub(super) fn begin_read(&mut self) -> Result<(), win32::Error> {
        let (ptr, length) = match self.read_state {
            ReadState::Header => (
                self.header_buffer[self.read_length..].as_mut_ptr(),
                self.header_buffer.len() - self.read_length,
            ),
            ReadState::Body => (
                self.body_buffer[self.read_length..].as_mut_ptr(),
                self.body_buffer.len() - self.read_length,
            ),
        };

        let result = unsafe {
            ReadFile(
                self.handle as _,
                ptr.cast(),
                length as _,
                null_mut(),
                self.read_overlapped.as_mut(),
            )
        };
        if result == FALSE {
            let error = unsafe { GetLastError() } as i32;
            if error != ERROR_IO_PENDING {
                return Err(win32::Error::new(error));
            }
        }

        Ok(())
    }
}
