mod bool;
mod error;
mod number;
mod parser;
mod string;

pub(super) use parser::Parser;

pub use error::ParseMessageError;

/// A parsable element
pub(in crate::messages) trait Parse: 'static + Sized {
    /// Parse this element from `parser`
    fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError>;
}
