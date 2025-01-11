use crate::messages::{Parse, ParseMessageError, Parser, StandClientMessage};

impl Parse for StandClientMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        if u8::parse(parser)? != 0 {
            return Err(ParseMessageError);
        }

        Ok(StandClientMessage {})
    }
}
