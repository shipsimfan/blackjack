use crate::messages::{GameStateServerMessage, Parse, ParseMessageError, Parser};

impl<'a> Parse for GameStateServerMessage<'a> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        parser.parse().map(GameStateServerMessage::Owned)
    }
}
