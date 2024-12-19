mod empty;
mod new;
mod next;
mod parse;

/// A parser for messages
pub(in crate::messages) struct Parser<'a> {
    /// The content being parsed
    content: &'a [u8],

    /// The current offset into the content
    offset: usize,
}
