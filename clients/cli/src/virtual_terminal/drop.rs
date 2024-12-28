use crate::VirtualTerminal;

impl Drop for VirtualTerminal {
    fn drop(&mut self) {
        self.use_main_buffer();
        self.show_cursor();
    }
}
