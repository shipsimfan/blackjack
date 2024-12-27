use crate::{virtual_terminal::SpecialKey, TerminalEvent, TextInput, VirtualTerminal};

impl TextInput {
    /// Handle a terminal event, returning if a value was entered
    pub fn handle_event(
        &mut self,
        event: &TerminalEvent,
        terminal: &mut VirtualTerminal,
    ) -> Option<&str> {
        let c = match event {
            TerminalEvent::Character(c) => *c,
            TerminalEvent::SpecialKey(SpecialKey::Delete) => {
                if self.cursor == self.value.len() {
                    return None;
                }

                self.value.remove(self.cursor);
                self.render(terminal);
                return None;
            }
            TerminalEvent::SpecialKey(SpecialKey::LeftArrow) => {
                if self.cursor == 0 {
                    return None;
                }

                self.cursor -= 1;
                self.render(terminal);
                return None;
            }
            TerminalEvent::SpecialKey(SpecialKey::RightArrow) => {
                if self.cursor == self.value.len() {
                    return None;
                }

                self.cursor += 1;
                self.render(terminal);
                return None;
            }
            TerminalEvent::SpecialKey(SpecialKey::Home) => {
                self.cursor = 0;
                self.render(terminal);
                return None;
            }
            TerminalEvent::SpecialKey(SpecialKey::End) => {
                self.cursor = self.value.len();
                self.render(terminal);
                return None;
            }
            TerminalEvent::Resize => {
                self.resize(terminal);
                self.render(terminal);
                return None;
            }
            _ => return None,
        };

        if c == '\r' {
            return Some(unsafe { std::str::from_utf8_unchecked(&self.value) });
        }

        if c == '\x08' {
            if self.cursor == 0 {
                return None;
            }

            self.cursor -= 1;
            self.value.remove(self.cursor);

            self.render(terminal);

            return None;
        }

        if !c.is_ascii_graphic() && c != ' ' {
            return None;
        }

        if self.value.len() >= self.max_length {
            return None;
        }

        self.value.insert(self.cursor, c as u8);
        self.cursor += 1;
        self.render(terminal);

        None
    }
}
