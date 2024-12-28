use crate::{SpecialKey, TerminalEvent, VirtualTerminal};
use win32::{
    try_get_last_error, ReadConsoleInput, FALSE, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD, WCHAR,
    WINDOW_BUFFER_SIZE_EVENT, WINDOW_BUFFER_SIZE_RECORD,
};

impl VirtualTerminal {
    /// Read an event from the virtual terminal
    pub fn read(&mut self) -> win32::Result<TerminalEvent> {
        let mut record = INPUT_RECORD::default();
        let mut events_read = 0;
        try_get_last_error!(ReadConsoleInput(
            self.input,
            &mut record,
            1,
            &mut events_read
        ))?;

        Ok(match record.event_type {
            KEY_EVENT => self.handle_key_event(unsafe { record.event.key_event }),
            WINDOW_BUFFER_SIZE_EVENT => {
                self.handle_resize_event(unsafe { record.event.window_buffer_size_event })
            }
            _ => TerminalEvent::Ignored,
        })
    }

    fn handle_key_event(&mut self, event: KEY_EVENT_RECORD) -> TerminalEvent {
        if event.key_down == FALSE {
            return TerminalEvent::Ignored;
        }

        let c = match char::from_u32(event.unicode_char as _) {
            Some(c) => c,
            None => {
                self.write(format_args!(
                    "Unknown character {}\n",
                    event.virtual_key_code
                ));
                return TerminalEvent::Ignored;
            }
        };

        if c == '\x00' {
            return SpecialKey::from_key_code(event.virtual_key_code)
                .map(TerminalEvent::SpecialKey)
                .unwrap_or(TerminalEvent::Ignored);
        }

        if c == '\x03' {
            return TerminalEvent::Exit;
        }

        TerminalEvent::Character(c)
    }

    fn handle_resize_event(&mut self, event: WINDOW_BUFFER_SIZE_RECORD) -> TerminalEvent {
        self.width = event.size.x as _;
        self.height = event.size.y as _;

        if self.blank_line.len() < self.width {
            self.blank_line
                .extend(std::iter::repeat(b' ' as WCHAR).take(self.width - self.blank_line.len()));
        }

        TerminalEvent::Resize
    }
}
