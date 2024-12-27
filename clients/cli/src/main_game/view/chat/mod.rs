use crate::TextInput;
use std::collections::VecDeque;

mod full_render;
mod new;
mod resize;

/// Displays the chat in the terminal
pub struct ChatView {
    /// The history of previously seen chat messages
    history: VecDeque<String>,

    /// The width of the chat area
    width: usize,

    /// The height of the chat area, excluding the text input
    height: usize,

    /// The rendered chat messages
    rendered: Vec<String>,

    /// The input for chat messages
    input: TextInput,
}

/// The maximum number of previous messages to store
const MAX_CHAT_MESSAGES: usize = 128;
