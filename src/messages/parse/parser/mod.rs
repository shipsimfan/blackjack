mod empty;
mod new;
mod next;
mod parse;
mod peek;

/// A parser for messages
pub(crate) struct Parser<'a> {
    /// The content being parsed
    content: &'a [u8],

    /// The current offset into the content
    offset: usize,
}
