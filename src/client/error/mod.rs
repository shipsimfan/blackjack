use crate::messages::{ErrorServerMessage, ParseMessageError};

mod display;

/// An error that can occur while running a client
#[derive(Debug)]
pub enum ClientError<E: std::error::Error> {
    /// The arguments were unable to be parsed
    ArgumentParseError(argparse::Error),

    /// The AI was unable to be created
    CreationError(E),

    /// Unable to connect to the server
    ConnectError(std::io::Error, String, u16),

    /// An error that occured while reading a message from the socket
    ReadIOError(std::io::Error),

    /// An error occured while parsing a message
    ParseMessageError(ParseMessageError),

    /// Received an unexpected message from the server
    UnexpectedMessage,

    /// No password was provided but the server required one
    NoPassword,

    /// The username provided is not valid
    InvalidUsername,

    /// An error occured while sending a message
    WriteIOError(std::io::Error),

    /// An error reported by the server
    ServerError(ErrorServerMessage),
}

impl<E: std::error::Error> std::error::Error for ClientError<E> {}
