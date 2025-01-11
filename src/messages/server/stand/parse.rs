use crate::messages::{Parse, ParseMessageError, Parser, StandServerMessage};

impl Parse for StandServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        if u8::parse(parser)? != 0 {
            return Err(ParseMessageError);
        }

        Ok(StandServerMessage {})
    }
}
