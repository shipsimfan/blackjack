use crate::{
    messages::{Parse, ParseMessageError, Parser},
    model::Player,
};

impl Parse for Player {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(Player {
            username: parser.parse()?,
            ai: parser.parse()?,
            state: parser.parse()?,
            hands: parser.parse()?,
        })
    }
}

impl Parse for Option<Player> {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        match parser.peek() {
            Some(0) => {
                parser.next();
                Ok(None)
            }
            Some(_) => Ok(Some(parser.parse()?)),
            None => Err(ParseMessageError),
        }
    }
}
