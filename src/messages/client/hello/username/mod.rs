use std::borrow::Cow;

mod borrow;
mod deref;
mod display;
mod generate;
mod new;
mod parse;
mod to_static;

/// A valid username
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username<'a> {
    /// The string representation of the username
    inner: Cow<'a, str>,
}
