use crate::TextInput;

mod full_render;
mod new;
mod resize;

/// Displays the chat in the terminal
pub struct ChatView {
    /// The width of the chat area
    width: usize,

    /// The height of the chat area, excluding the text input
    height: usize,

    /// The input for chat messages
    input: TextInput,
}

/// The maximum number of previous messages to store
const MAX_CHAT_MESSAGES: usize = 128;
