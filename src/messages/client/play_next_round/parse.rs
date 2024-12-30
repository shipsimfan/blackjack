use crate::messages::{Parse, ParseMessageError, Parser, PlayNextRoundClientMessage};

impl Parse for PlayNextRoundClientMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(PlayNextRoundClientMessage {
            play_next_round: parser.parse()?,
        })
    }
}
