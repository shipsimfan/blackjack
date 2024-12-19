use crate::messages::{Parse, ParseMessageError, Parser};

impl<'a> Parser<'a> {
    /// Parse `T` from the content
    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseMessageError> {
        T::parse(self)
    }
}
