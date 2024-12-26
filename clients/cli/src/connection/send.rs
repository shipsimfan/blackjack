use std::ptr::null_mut;

use crate::Connection;
use blackjack::messages::ClientMessage;
use win32::{
    try_get_last_error, CancelIo, GetOverlappedResult, WriteFile, OVERLAPPED, OVERLAPPED_UNION,
    TRUE,
};

impl Connection {
    /// Sends a `message` to the server
    pub fn send(&mut self, message: ClientMessage) {
        try_get_last_error!(CancelIo(self.handle as _)).unwrap();

        message.generate(&mut self.write_buffer);

        let mut total_bytes_written = 0;
        while total_bytes_written < self.write_buffer.len() {
            let mut overlapped = OVERLAPPED {
                internal: 0,
                internal_high: 0,
                union: OVERLAPPED_UNION {
                    pointer: null_mut(),
                },
                event: null_mut(),
            };

            try_get_last_error!(WriteFile(
                self.handle as _,
                self.write_buffer[total_bytes_written..].as_ptr().cast(),
                (self.write_buffer.len() - total_bytes_written) as _,
                null_mut(),
                &mut overlapped,
            ))
            .map_err(|error| {
                eprintln!("{}", error);
                error
            })
            .unwrap();

            let mut bytes_written = 0;
            try_get_last_error!(GetOverlappedResult(
                self.handle as _,
                &mut overlapped,
                &mut bytes_written,
                TRUE
            ))
            .unwrap();

            if bytes_written == 0 {
                panic!()
            }

            total_bytes_written += bytes_written as usize;
        }
    }
}
