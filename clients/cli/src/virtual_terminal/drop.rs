use crate::VirtualTerminal;

impl Drop for VirtualTerminal {
    fn drop(&mut self) {
        if self.using_alt_buffer {
            self.use_main_buffer();
        }
        self.show_cursor();
    }
}
