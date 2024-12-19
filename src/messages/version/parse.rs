use crate::messages::{Parse, ParseMessageError, Parser, Version};

impl Parse for Version {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(Version {
            major: parser.parse()?,
            minor: parser.parse()?,
            patch: parser.parse()?,
        })
    }
}
