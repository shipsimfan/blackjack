use crate::messages::Parser;

impl<'a> Parser<'a> {
    /// Has the content run out of bytes?
    pub fn empty(&self) -> bool {
        self.offset >= self.content.len()
    }
}
