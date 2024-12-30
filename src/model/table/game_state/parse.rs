use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::GameState,
};

impl Parse for GameState {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        match u8::parse(parser)? {
            0u8 => Ok(GameState::WaitingForPlayers),
            1u8 => Ok(GameState::WaitingForBets),
            2u8 => Ok(GameState::WaitingForPlayer(parser.parse()?)),
            _ => Err(ParseMessageError),
        }
    }
}
