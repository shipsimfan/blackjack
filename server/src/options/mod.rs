use argparse::Command;
use blackjack::messages::DEFAULT_PORT;
use std::{
    net::IpAddr,
    num::{NonZeroU16, NonZeroU8},
};

mod get;

/// The options for the game and server
#[derive(Command)]
#[command(help, version)]
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
    #[flag(value = "TIMEOUT", default = DEFAULT_CONNECTION_TIMEOUT, description = "The number of seconds to allow a player to connect before timing out. Defaults to 300 seconds")]
    connection_timeout: NonZeroU16,

    /// The name of the server to report to clients
    #[flag(
        value = "NAME",
        description = "The name of the server to report to clients"
    )]
    pub server_name: Option<String>,

    /// A password to require clients to enter to connect
    #[flag(
        value = "PASSWORD",
        description = "Password required by clients to connect"
    )]
    pub password: Option<String>,

    /// The maximum amount one player can bet on one hand in one round
    #[flag(
        value = "BET",
        default = DEFAULT_MAX_BET,
        description = "The maximum amount a player can bet on one hand in one round. Defaults to $1000."
    )]
    pub max_bet: NonZeroU16,

    /// The minimum amount a player must bet on one hand in one round
    #[flag(
        value = "BET",
        default = DEFAULT_MIN_BET,
        description = "The minimum amount a player must bet on one hand in one round. Defaults to $10."
    )]
    pub min_bet: NonZeroU16,
}

/// The default maximum number of players that can connect
const DEFAULT_MAX_PLAYERS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };

/// The default timeout to reject clients if they haven't returned a hello message
const DEFAULT_CONNECTION_TIMEOUT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(300) };

/// The default maximum amount a player can bet for hand in a single round
const DEFAULT_MAX_BET: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(1000) };

/// The default minimum amount a player must bet for a hand in a single round
const DEFAULT_MIN_BET: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(10) };
