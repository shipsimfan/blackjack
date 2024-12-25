use crate::messages::{
    HelloServerMessage, Parse, ParseMessageError, Parser, CURRENT_PROTOCOL_VERSION,
};

impl<'a> Parse for HelloServerMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let protocol_version = parser.parse()?;
        if protocol_version != CURRENT_PROTOCOL_VERSION {
            return Err(ParseMessageError);
        }

        Ok(HelloServerMessage {
            protocol_version,
            password_required: parser.parse()?,
            server_name: parser.parse()?,
            server_application_name: parser.parse()?,
            server_version: parser.parse()?,
        })
    }
}
