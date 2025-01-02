use crate::messages::{Parse, ParseMessageError, Parser};

impl<T1: Parse, T2: Parse> Parse for (T1, T2) {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok((parser.parse()?, parser.parse()?))
    }
}
