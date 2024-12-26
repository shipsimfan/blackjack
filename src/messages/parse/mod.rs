mod bool;
mod collections;
mod cow;
mod error;
mod number;
mod parser;
mod string;

pub(crate) use parser::Parser;

pub use error::ParseMessageError;

/// A parsable element
pub(crate) trait Parse: Sized {
    /// Parse this element from `parser`
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError>;
}
