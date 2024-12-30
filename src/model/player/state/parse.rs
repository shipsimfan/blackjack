use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::PlayerState,
};

impl Parse for PlayerState {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(match u8::parse(parser)? {
            0 => PlayerState::NotPlaying,
            1 => PlayerState::PlayingNextRound,
            2 => PlayerState::PlayingThisRound,
            _ => return Err(ParseMessageError),
        })
    }
}
