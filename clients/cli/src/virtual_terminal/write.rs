use crate::VirtualTerminal;
use std::ptr::null_mut;
use win32::{try_get_last_error, WriteFile};

impl VirtualTerminal {
    /// Write a value to the virtual terminal
    pub fn write<T: std::fmt::Display>(&mut self, value: T) {
        let string = value.to_string();
        let mut bytes_written = 0;

        while bytes_written < string.len() {
            let mut bytes = 0;
            try_get_last_error!(WriteFile(
                self.output,
                string.as_ptr().cast(),
                (string.len() - bytes_written) as _,
                &mut bytes,
                null_mut()
            ))
            .unwrap();

            bytes_written += bytes as usize;
        }
    }
}
