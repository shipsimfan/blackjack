use crate::messages::{GameStateServerMessage, Parse, ParseMessageError, Parser};

impl<'a> Parse for GameStateServerMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(GameStateServerMessage {
            client_id: parser.parse()?,
            table: parser.parse()?,
        })
    }
}
