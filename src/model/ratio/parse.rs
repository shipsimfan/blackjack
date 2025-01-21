use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::Ratio,
};

impl Parse for Ratio {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(Ratio {
            numerator: parser.parse()?,
            denominator: parser.parse()?,
        })
    }
}
