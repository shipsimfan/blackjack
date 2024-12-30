mod clear;
mod cursor_xy;
mod get;
mod handle_event;
mod new;
mod render;
mod resize;

/// A utility for getting a users text input from a virtual terminal
pub struct TextInput {
    /// The currently contained value
    value: Vec<u8>,

    /// The current index of the cursor
    cursor: usize,

    /// The position of the cursor at last render
    last_cursor: usize,

    /// The current width of the first line of input
    first_line_width: usize,

    /// The current width of the text-input
    width: usize,

    /// The current height of the text-input
    height: usize,

    /// The maximum length of the input string
    max_length: usize,

    /// Should the value be hidden?
    hide_character: Option<u8>,

    /// The prompt to display for text input
    prompt: &'static str,

    /// The margin from the left of the screen for this input
    margin: usize,

    /// Should this input be displayed at the bottom of the terminal or on the current line?
    at_bottom: bool,

    /// The maximum width this text input can be
    max_width: Option<usize>,

    /// A predicate to be able to reject characters from entering the input
    predicate: Option<fn(char) -> bool>,
}
