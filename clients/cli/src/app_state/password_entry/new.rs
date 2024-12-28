use super::PasswordEntryState;
use crate::{virtual_terminal::TextInput, VirtualTerminal};
use blackjack::messages::Username;

impl PasswordEntryState {
    pub fn new(
        username: Username<'static>,
        server_name: String,
        terminal: &mut VirtualTerminal,
    ) -> Self {
        PasswordEntryState {
            username,
            password_input: TextInput::new(
                u8::MAX as usize,
                "Password: ",
                0,
                Some('*'),
                false,
                true,
                Some(terminal),
            ),
            server_name,
        }
    }
}
