use argparse::Command;
use std::{
    net::IpAddr,
    num::{NonZeroU16, NonZeroU8},
};

mod get;

/// The options for the game and server
#[derive(Command)]
#[command(help, version, description = "Server for hosting Blackjack games")]
pub struct Options {
    /// The address to listen for clients on
    #[flag(
        value = "ADDRESS",
        description = "The address to listen for clients on"
    )]
    address: Option<IpAddr>,

    /// The port to listen for clients on
    #[flag(
        short_name,
        value = "PORT",
        default = DEFAULT_PORT,
        description = "The port to listen for clients on. Defaults to 9261"
    )]
    port: NonZeroU16,

    /// The maximum number of players that can connect at one time
    #[flag(value = "PLAYERS", default = DEFAULT_MAX_PLAYERS, description = "The maximum number of players that can connect at one time. Defaults to 8")]
    max_players: NonZeroU8,

    /// The number of seconds to allow a player to connect before timing out
    #[flag(value = "TIMEOUT", default = DEFAULT_CONNECTION_TIMEOUT, description = "The number of seconds to allow a player to connect before timing out. Defaults to 60 seconds")]
    connection_timeout: NonZeroU16,
}

/// The default port to listen for clients on
const DEFAULT_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(9261) };

/// The default maximum number of players that can connect
const DEFAULT_MAX_PLAYERS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };

const DEFAULT_CONNECTION_TIMEOUT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(60) };
