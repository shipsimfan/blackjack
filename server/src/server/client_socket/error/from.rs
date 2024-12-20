use super::ReadMessageError;
use blackjack::messages::ParseMessageError;

impl From<linux::Error> for ReadMessageError {
    fn from(error: linux::Error) -> Self {
        ReadMessageError::IO(error)
    }
}

impl From<ParseMessageError> for ReadMessageError {
    fn from(error: ParseMessageError) -> Self {
        ReadMessageError::Parse(error)
    }
}
