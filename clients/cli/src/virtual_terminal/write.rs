use crate::VirtualTerminal;
use std::ptr::null_mut;
use win32::{try_get_last_error, WriteConsole};

impl VirtualTerminal {
    /// Write a value to the virtual terminal
    pub fn write<T: std::fmt::Display>(&mut self, value: T) {
        let string: Vec<_> = value.to_string().encode_utf16().collect();
        let mut chars_written = 0;

        while chars_written < string.len() {
            let mut chars = 0;
            try_get_last_error!(WriteConsole(
                self.output,
                string[chars_written..].as_ptr(),
                (string.len() - chars_written) as _,
                &mut chars,
                null_mut()
            ))
            .unwrap();

            chars_written += chars as usize;
        }
    }

    /// Writes `count` spaces
    pub fn write_blank(&mut self, count: usize) {
        assert!(count <= self.width);

        let mut chars_written = 0;
        while chars_written < count {
            let mut chars = 0;
            try_get_last_error!(WriteConsole(
                self.output,
                self.blank_line.as_ptr(),
                (count - chars_written) as _,
                &mut chars,
                null_mut()
            ))
            .unwrap();

            chars_written += chars as usize;
        }
    }
}
