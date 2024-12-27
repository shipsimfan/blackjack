use crate::messages::{ClientConnectedServerMessage, Parse, ParseMessageError, Parser};

impl<'a> Parse for ClientConnectedServerMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(ClientConnectedServerMessage {
            player: parser.parse()?,
        })
    }
}
