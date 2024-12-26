use crate::messages::Parser;

impl<'a> Parser<'a> {
    /// Gets the next element in the content with advancing
    pub fn peek(&self) -> Option<u8> {
        self.content.get(self.offset).map(|byte| *byte)
    }
}
