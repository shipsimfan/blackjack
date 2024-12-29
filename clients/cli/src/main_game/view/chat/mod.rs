use crate::TextInput;
use std::collections::VecDeque;

mod add_message;
mod full_render;
mod new;
mod render;
mod resize;

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
