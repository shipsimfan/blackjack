use crate::{TextInput, VirtualTerminal};

impl TextInput {
    /// Clears the input value
    pub fn clear(&mut self, terminal: &mut VirtualTerminal) {
        self.value.truncate(0);
        self.cursor = 0;
        self.render(true, terminal);
    }
}
