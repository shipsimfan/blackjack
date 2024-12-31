use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::Hand,
};

impl Parse for Hand {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(Hand {
            bet: parser.parse()?,
            cards: parser.parse()?,
        })
    }
}
