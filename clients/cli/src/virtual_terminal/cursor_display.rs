use crate::VirtualTerminal;

impl VirtualTerminal {
    /// Show the cursor
    pub fn show_cursor(&mut self) {
        self.write("\x1B[25h");
    }

    /// Hide the cursor
    pub fn hide_cursor(&mut self) {
        self.write("\x1B[25l");
    }
}
