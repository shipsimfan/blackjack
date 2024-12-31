use crate::messages::{Parse, ParseMessageError, Parser, PlaceBetServerMessage};

impl Parse for PlaceBetServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(PlaceBetServerMessage {
            client: parser.parse()?,
            bet: parser.parse()?,
        })
    }
}
