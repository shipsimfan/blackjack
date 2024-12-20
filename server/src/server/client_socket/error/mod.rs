use blackjack::messages::ParseMessageError;

mod display;
mod from;

/// An error that can occur while reading a message
#[derive(Debug)]
pub enum ReadMessageError {
    /// An I/O error occurred
    IO(linux::Error),

    /// A message was unable to be parsed
    Parse(ParseMessageError),
}

impl std::error::Error for ReadMessageError {}
