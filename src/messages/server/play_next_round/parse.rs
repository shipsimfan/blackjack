use crate::messages::{Parse, ParseMessageError, Parser, PlayNextRoundServerMessage};

impl Parse for PlayNextRoundServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(PlayNextRoundServerMessage {
            client: parser.parse()?,
            play_next_round: parser.parse()?,
        })
    }
}
