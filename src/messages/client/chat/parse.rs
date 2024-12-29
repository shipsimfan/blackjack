use crate::messages::{ChatClientMessage, Parse, ParseMessageError, Parser};

impl<'a> Parse for ChatClientMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(ChatClientMessage {
            message: parser.parse()?,
        })
    }
}
