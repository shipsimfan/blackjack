mod display;

/// An error occured while parsing a message
#[derive(Debug)]
pub struct ParseMessageError;

impl std::error::Error for ParseMessageError {}
