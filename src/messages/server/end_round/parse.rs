use crate::messages::{EndRoundServerMessage, Parse, ParseMessageError, Parser};

impl Parse for EndRoundServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(EndRoundServerMessage {
            dealer_card: parser.parse()?,
            dealer_play: parser.parse()?,
        })
    }
}
