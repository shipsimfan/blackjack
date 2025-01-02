use crate::messages::{DealServerMessage, Parse, ParseMessageError, Parser};

impl Parse for DealServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(DealServerMessage {
            dealer_up_card: parser.parse()?,
            dealer_down_card: if let Some(0) = parser.peek() {
                u8::parse(parser).unwrap();
                None
            } else {
                Some(parser.parse()?)
            },
            hands: parser.parse()?,
        })
    }
}
