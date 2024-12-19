use crate::messages::Parser;

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] for `content`
    pub fn new(content: &'a [u8]) -> Self {
        Parser { content, offset: 0 }
    }
}
