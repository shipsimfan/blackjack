use super::ChatView;
use crate::TextInput;

impl ChatView {
    /// Creates a new [`ChatView`] without rendering it
    pub fn new(margin: usize) -> Self {
        let input = TextInput::new(u8::MAX as usize, "Chat: ", margin, None, true, false, None);
        ChatView {
            width: 0,
            height: 0,
            input,
        }
    }
}
