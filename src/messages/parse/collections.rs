use crate::messages::{Parse, ParseMessageError, Parser};

impl<T: Parse> Parse for Vec<T> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let length = u8::parse(parser)?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(parser.parse()?);
        }
        Ok(vec)
    }
}

impl<T: Parse> Parse for Box<[T]> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Vec::parse(parser).map(Vec::into_boxed_slice)
    }
}
