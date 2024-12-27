use crate::messages::{Parse, ParseMessageError, Parser, RefCow};

impl<'a, T: Parse> Parse for RefCow<'a, T> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        parser.parse().map(RefCow::Owned)
    }
}
