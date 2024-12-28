use crate::VirtualTerminal;

impl Drop for VirtualTerminal {
    fn drop(&mut self) {
        self.show_cursor();
    }
}
