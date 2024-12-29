use crate::messages::{ErrorServerMessage, Parse, ParseMessageError, Parser};

impl Parse for ErrorServerMessage {
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
        Ok(match u32::parse(parser)? {
            0 => ErrorServerMessage::ServerFull,
            1 => ErrorServerMessage::ConnectionTimeout,
            2 => ErrorServerMessage::InvalidPassword,
            3 => ErrorServerMessage::UsernameTaken,
            _ => return Err(ParseMessageError),
        })
    }
}
