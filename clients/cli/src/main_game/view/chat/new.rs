use super::{ChatView, MAX_CHAT_MESSAGES};
use crate::TextInput;
use std::collections::VecDeque;

impl ChatView {
    /// Creates a new [`ChatView`] without rendering it
    pub fn new(margin: usize) -> Self {
        let input = TextInput::new(u8::MAX as usize, "", margin, None, true, false, None);
        ChatView {
            history: VecDeque::with_capacity(MAX_CHAT_MESSAGES),
            rendered: VecDeque::new(),
            width: 0,
            height: 0,
            input,
        }
    }
}
