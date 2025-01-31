use argparse::Command;
use std::num::NonZeroU16;

/// Options to be passed to an AI
pub trait Options: Command {
    /// Get the address to connect to
    fn address(&self) -> &str;

    /// Get the port to connect on
    fn port(&self) -> NonZeroU16;

    /// The username to present as
    fn username(&self) -> &str;

    /// The password to connect with
    fn password(&self) -> Option<&str>;
}
