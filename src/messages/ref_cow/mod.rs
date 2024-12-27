mod deref;
mod from;
mod generate;
mod parse;
mod to_static;
mod unwrap;

/// An item which can either be borrowed or owned
#[derive(Debug, Clone)]
pub enum RefCow<'a, T> {
    /// The item is borrowed
    Borrowed(&'a T),

    /// The item is owned
    Owned(T),
}
