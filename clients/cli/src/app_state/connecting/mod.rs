use crate::Options;

mod handle_message;
mod handle_terminal;
mod new;
mod unwrap;

/// Currently connecting to the server
pub struct Connecting {
    /// The options used to create the program
    options: Options,
}
