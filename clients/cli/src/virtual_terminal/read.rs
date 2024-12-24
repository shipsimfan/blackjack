use crate::VirtualTerminal;
use win32::{try_get_last_error, ReadConsoleInput, INPUT_RECORD};

impl VirtualTerminal {
    /// Read an event from the virtual terminal
    pub fn read(&mut self) -> win32::Result<()> {
        let mut record = INPUT_RECORD::default();
        let mut events_read = 0;
        try_get_last_error!(ReadConsoleInput(
            self.input,
            &mut record,
            1,
            &mut events_read
        ))?;

        Ok(())
    }
}
