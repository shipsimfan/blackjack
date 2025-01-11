use crate::messages::{HitClientMessage, Parse, ParseMessageError, Parser};

impl Parse for HitClientMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        if u8::parse(parser)? != 0 {
            return Err(ParseMessageError);
        }

        Ok(HitClientMessage {})
    }
}
