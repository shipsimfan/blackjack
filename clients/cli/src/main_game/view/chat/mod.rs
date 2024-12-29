use crate::TextInput;
use std::collections::VecDeque;

mod add_message;
mod chat_event;
mod full_render;
mod handle_terminal;
mod move_cursor;
mod new;
mod render;
mod resize;
mod set_active;

pub use chat_event::ChatEvent;

/// Displays the chat in the terminal
pub struct ChatView {
    /// The list of previously seen messages
    history: VecDeque<String>,

    /// The currently rendered messages
    rendered: VecDeque<String>,

    /// The width of the chat area
    width: usize,

    /// The height of the chat area, excluding the text input
    height: usize,

    /// The input for chat messages
    input: TextInput,
}

/// The maximum number of previous messages to store
const MAX_CHAT_MESSAGES: usize = 128;
