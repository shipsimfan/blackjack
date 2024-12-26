use crate::{TextInput, VirtualTerminal};

impl TextInput {
    /// Create a new [`TextInput`]
    pub fn new(
        max_length: usize,
        prompt: &'static str,
        margin: usize,
        hide_character: Option<char>,
        at_bottom: bool,
        terminal: &mut VirtualTerminal,
    ) -> Self {
        let mut input = TextInput {
            value: Vec::with_capacity(max_length),
            first_line_width: 0,
            width: 0,
            height: 0,
            cursor: 0,
            last_cursor: 0,

            max_length,
            hide_character: hide_character.map(|char| char as u8),
            prompt,
            margin,
            at_bottom,
        };

        input.recalculate_bounds(terminal);
        input.render(terminal);

        input
    }
}