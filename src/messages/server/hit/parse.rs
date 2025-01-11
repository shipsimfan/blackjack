use crate::messages::{HitServerMessage, Parse, ParseMessageError, Parser};

impl Parse for HitServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(HitServerMessage {
            card: parser.parse()?,
        })
    }
}
