use super::PasswordEntryState;
use crate::{virtual_terminal::TextInput, Options, VirtualTerminal};

impl PasswordEntryState {
    pub fn new(options: Options, virtual_terminal: &mut VirtualTerminal) -> Self {
        PasswordEntryState {
            options,
            password_input: TextInput::new(
                u8::MAX as usize,
                "Password: ",
                0,
                Some('*'),
                false,
                virtual_terminal,
            ),
        }
    }
}
