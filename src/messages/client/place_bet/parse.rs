use crate::messages::{Parse, ParseMessageError, Parser, PlaceBetClientMessage};

impl Parse for PlaceBetClientMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(PlaceBetClientMessage {
            bet: parser.parse()?,
        })
    }
}
