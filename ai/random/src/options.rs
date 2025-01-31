use argparse::Command;
use blackjack::messages::DEFAULT_PORT;
use std::num::NonZeroU16;

/// The options the user selected for the AI
#[derive(Command)]
#[command(version, help)]
pub struct Options {
    /// The address to connect to
    #[arg(description = "The address to connect to")]
    address: String,

    /// The username for the AI to present as
    #[arg(description = "The username for the AI to present as")]
    username: String,

    /// The port to connect to
    #[flag(short_name, default = DEFAULT_PORT, description = "The port to connect to. Defaults to 9261")]
    port: NonZeroU16,

    /// The password to use when connecting to the server
    #[flag(description = "The password to use whe connecting to the server")]
    password: Option<String>,

    /// The percentage chance to hit on a hand, from 0 - 1
    #[flag(
        short_name,
        description = "The percentage chance to hit on a hand, from 0 - 1"
    )]
    pub hit_chance: Option<f32>,

    /// The amount of money to bet each round
    #[flag(
        short_name,
        default = DEFAULT_BET,
        description = "The amount of money to bet each round. Defaults to 10"
    )]
    pub bet: NonZeroU16,
}

/// The default amount of money to bet each round
const DEFAULT_BET: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(10) };

impl blackjack::client::Options for Options {
    fn address(&self) -> &str {
        &self.address
    }

    fn port(&self) -> NonZeroU16 {
        self.port
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> Option<&str> {
        self.password.as_ref().map(String::as_str)
    }
}
