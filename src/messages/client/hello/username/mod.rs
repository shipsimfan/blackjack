mod deref;
mod display;
mod new;
mod parse;

/// A valid username
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username {
    /// The string representation of the username
    inner: String,
}
