use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::BlackjackTable,
};

impl Parse for BlackjackTable {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(BlackjackTable {
            shoe: None,
            players: parser.parse()?,
            state: parser.parse()?,
            dealer_hand: parser.parse()?,
            max_bet: parser.parse()?,
            min_bet: parser.parse()?,
            min_players: parser.parse()?,
            min_humans: parser.parse()?,
            max_hands: parser.parse()?,
            blackjack_payout: parser.parse()?,
            hit_soft_17: parser.parse()?,
        })
    }
}
