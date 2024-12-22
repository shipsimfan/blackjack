use crate::messages::{HelloServerMessage, Parse, ParseMessageError, Parser};

impl Parse for HelloServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(HelloServerMessage {
            protocol_version: parser.parse()?,
            password_required: parser.parse()?,
            server_name: parser.parse()?,
            server_application_name: parser.parse()?,
            server_version: parser.parse()?,
        })
    }
}
