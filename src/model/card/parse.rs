use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::{Card, Rank, Suit},
};

impl Parse for Card {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        let value = u8::parse(parser)?;
        let rank = Rank::from_u8(value / Suit::ALL.len() as u8).ok_or(ParseMessageError)?;
        let suit = Suit::from_u8(value % Suit::ALL.len() as u8).ok_or(ParseMessageError)?;
        Ok(Card { rank, suit })
    }
}

impl Parse for Option<Card> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(if let Some(0) = parser.peek() {
            u8::parse(parser).unwrap();
            None
        } else {
            Some(parser.parse()?)
        })
    }
}
