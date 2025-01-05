use crate::messages::{Parse, ParseMessageError, Parser, ShuffleServerMessage};

impl Parse for ShuffleServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        if u8::parse(parser)? != 0 {
            return Err(ParseMessageError);
        }

        Ok(ShuffleServerMessage {})
    }
}
