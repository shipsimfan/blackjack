use crate::messages::{ClientDisconnectedServerMessage, Parse, ParseMessageError, Parser};

impl Parse for ClientDisconnectedServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(ClientDisconnectedServerMessage {
            id: parser.parse()?,
        })
    }
}
