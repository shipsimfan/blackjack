use crate::messages::{ParseMessageError, Parser};

impl<'a> Parser<'a> {
    /// Gets the next byte in the content, if there is one
    pub fn next(&mut self) -> Option<u8> {
        let ret = self.content.get(self.offset).map(|val| *val);
        self.offset += 1;
        ret
    }

    /// Gets the next byte in the content or returns an error if there is none
    pub fn next_err(&mut self) -> Result<u8, ParseMessageError> {
        self.next().ok_or(ParseMessageError)
    }

    /// Gets the next `count` bytes from the content, if there is enough available
    pub fn next_range(&mut self, count: usize) -> Option<&[u8]> {
        if self.offset + count > self.content.len() {
            return None;
        }

        let ret = &self.content[self.offset..self.offset + count];
        self.offset += count;
        Some(ret)
    }

    /// Gets the next `count` bytes from the content or returns an error if there is not sufficient bytes
    pub fn next_range_err(&mut self, count: usize) -> Result<&[u8], ParseMessageError> {
        self.next_range(count).ok_or(ParseMessageError)
    }
}
