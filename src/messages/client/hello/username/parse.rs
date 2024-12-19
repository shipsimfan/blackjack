use crate::messages::{Parse, ParseMessageError, Parser, Username};

impl Parse for Username {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Username::new(parser.parse()?).ok_or(ParseMessageError)
    }
}
