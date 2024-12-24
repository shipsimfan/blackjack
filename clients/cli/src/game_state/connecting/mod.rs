use crate::{Options, VirtualTerminal};

mod new;

/// Currently connecting to the server
pub(super) struct Connecting {
    /// The options used to create the program
    options: Options,
}
