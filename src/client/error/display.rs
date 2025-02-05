use crate::client::ClientError;

impl<E: std::error::Error> std::fmt::Display for ClientError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ArgumentParseError(error) => {
                write!(f, "unable to parse arguments - {error}")
            }
            ClientError::CreationError(error) => write!(f, "unable to create the AI - {error}"),
            ClientError::ConnectError(error, address, port) => {
                write!(f, "unable to connect to \"{address}:{port}\" - {error}")
            }
            ClientError::ReadIOError(error) => write!(f, "unable to read a message - {error}"),
            ClientError::ParseMessageError(error) => error.fmt(f),
            ClientError::UnexpectedMessage => {
                "received an unexpected message from the server".fmt(f)
            }
            ClientError::NoPassword => {
                "the server requires a password but none was provided".fmt(f)
            }
            ClientError::InvalidUsername => "the provided username is not valid".fmt(f),
            ClientError::WriteIOError(error) => write!(f, "unable to send a message - {error}"),
            ClientError::ServerError(error) => error.fmt(f),
        }
    }
}
