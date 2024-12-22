use crate::messages::{HelloClientMessage, Parse, ParseMessageError, Parser};

impl Parse for HelloClientMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(HelloClientMessage {
            username: parser.parse()?,
            password: parser.parse()?,
            client_name: parser.parse()?,
            client_version: parser.parse()?,
            ai: parser.parse()?,
        })
    }
}