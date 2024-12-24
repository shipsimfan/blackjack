use argparse::Command;
use blackjack::messages::DEFAULT_PORT;
use std::num::NonZeroU16;

mod get;

#[derive(Command)]
#[command(version, help)]
pub struct Options {
    /// The address to connect to
    #[arg(description = "The address to connect to")]
    address: String,

    /// The username to present as
    #[arg(description = "The username to present as")]
    username: String,

    /// The port to connect on
    #[flag(short_name, value = "PORT", default = DEFAULT_PORT, description = "The port to connect on")]
    port: NonZeroU16,
}
