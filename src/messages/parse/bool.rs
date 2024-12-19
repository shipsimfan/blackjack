use crate::messages::{Parse, ParseMessageError, Parser};

impl Parse for bool {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let data = u8::parse(parser)?;
        match data {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ParseMessageError),
        }
    }
}
