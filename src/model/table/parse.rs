use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::BlackjackTable,
};

impl Parse for BlackjackTable {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(BlackjackTable {
            players: parser.parse()?,
        })
    }
}
