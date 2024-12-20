use crate::messages::{DisconnectClientMessage, Parse, ParseMessageError, Parser};

impl Parse for DisconnectClientMessage {
    fn parse(_: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(DisconnectClientMessage)
    }
}
