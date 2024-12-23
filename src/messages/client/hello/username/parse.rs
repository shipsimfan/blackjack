use crate::messages::{Parse, ParseMessageError, Parser, Username};

impl<'a> Parse for Username<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Username::new(String::parse(parser)?).ok_or(ParseMessageError)
    }
}
