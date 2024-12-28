use crate::VirtualTerminal;

impl VirtualTerminal {
    /// Begin using the alternate buffer
    pub fn use_alt_buffer(&mut self) {
        self.write("\x1B[?1049h");
    }

    /// Return to the main buffer
    pub fn use_main_buffer(&mut self) {
        self.write("\x1B[?1049l");
    }
}
