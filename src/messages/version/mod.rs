mod display;
mod generate;
mod package;
mod parse;

/// The version of a client or server program
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// The reported major version
    pub major: u32,

    /// The reported minor version
    pub minor: u32,

    /// The reported patch
    pub patch: u32,
}
