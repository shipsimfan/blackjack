use std::borrow::Cow;

mod deref;
mod display;
mod generate;
mod new;
mod parse;

/// A valid username
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username<'a> {
    /// The string representation of the username
    inner: Cow<'a, str>,
}
