use crate::messages::{Parse, ParseMessageError, Parser};
use std::{borrow::Cow, num::NonZeroU8};

impl Parse for String {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let length = NonZeroU8::parse(parser)?.get();
        let content = parser.next_range_err(length as usize)?;
        std::str::from_utf8(content)
            .map(|content| content.to_owned())
            .map_err(|_| ParseMessageError)
    }
}

impl Parse for Option<String> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        match parser.peek() {
            Some(0) => {
                parser.next();
                Ok(None)
            }
            Some(_) => Ok(Some(parser.parse()?)),
            None => Err(ParseMessageError),
        }
    }
}

impl<'a> Parse for Option<Cow<'a, str>> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        parser
            .parse()
            .map(|value: Option<String>| value.map(Cow::Owned))
    }
}
