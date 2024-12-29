use crate::messages::{ChatServerMessage, Parse, ParseMessageError, Parser};

impl<'a> Parse for ChatServerMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(ChatServerMessage {
            client: parser.parse()?,
            message: parser.parse()?,
        })
    }
}
