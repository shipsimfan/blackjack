use crate::messages::{Parse, ParseMessageError, Parser};
use std::borrow::Cow;

impl<'a> Parse for Cow<'a, str> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        parser.parse().map(Cow::Owned)
    }
}
