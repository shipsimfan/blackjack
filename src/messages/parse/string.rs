use crate::messages::{Parse, ParseMessageError, Parser};

impl Parse for String {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let length = u8::parse(parser)?;
        let content = parser.next_range_err(length as usize)?;
        std::str::from_utf8(content)
            .map(|content| content.to_owned())
            .map_err(|_| ParseMessageError)
    }
}
