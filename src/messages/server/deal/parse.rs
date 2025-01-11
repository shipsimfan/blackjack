use crate::messages::{DealServerMessage, Parse, ParseMessageError, Parser};

impl Parse for DealServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(DealServerMessage {
            dealer_up_card: parser.parse()?,
            dealer_down_card: parser.parse()?,
            hands: parser.parse()?,
        })
    }
}
