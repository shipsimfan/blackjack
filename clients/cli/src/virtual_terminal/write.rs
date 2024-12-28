use crate::VirtualTerminal;
use std::ptr::null_mut;
use win32::{try_get_last_error, WriteFile};

impl VirtualTerminal {
    /// Write a value to the virtual terminal
    pub fn write<T: std::fmt::Display>(&mut self, value: T) {
        let string: Vec<_> = value.to_string().encode_utf16().collect();
        let mut bytes_written = 0;

        while bytes_written < string.len() {
            let mut bytes = 0;
            todo!("Change this to WriteConsoleW");
            try_get_last_error!(WriteFile(
                self.output,
                string.as_ptr().byte_add(bytes_written).cast(),
                (string.len() * 2 - bytes_written) as _,
                &mut bytes,
                null_mut()
            ))
            .unwrap();

            bytes_written += bytes as usize;
        }
    }
}
