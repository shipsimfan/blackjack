use argparse::Command;
use blackjack::{messages::DEFAULT_PORT, model::Ratio};
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
        short_name,
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
        short_name,
        value = "NAME",
        description = "The name of the server to report to clients"
    )]
    pub server_name: Option<String>,

    /// A password to require clients to enter to connect
    #[flag(
        short_name = 'P',
        value = "PASSWORD",
        description = "Password required by clients to connect"
    )]
    pub password: Option<String>,

    /// The number of decks to play in the shoe
    #[flag(short_name, value = "DECKS", default = DEFAULT_DECKS, description = "The number of decks to play in the shoe. Defaults to 6")]
    pub decks: NonZeroU8,

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

    /// The minimum number of players to begin a round
    #[flag(value = "PLAYERS", default = DEFUALT_MIN_PLAYERS, description = "The minimum number of players to begin a round. Defaults to 1.")]
    pub min_players: NonZeroU8,

    /// The minimum number of humans to begin a round
    #[flag(value = "HUMANS", default = DEFUALT_MIN_HUMANS, description = "The minimum number of human players to begin a round. Defaults to 0.")]
    pub min_humans: u8,

    /// The maximum number of hands a single player can start a round with
    ///
    /// NOTE: The actual number of hands a player is playing may exceed this value due to splitting
    #[flag(value = "HANDS", default = DEFAULT_MAX_HANDS, description = "The maximum number of hands a player can start a round with. Defaults to 3.")]
    pub max_hands: NonZeroU8,

    /// The ratio to payout for blackjack
    #[flag(short_name, value = "PAYOUT", default = DEFAULT_BLACKJACK_PAYOUT, description = "The ratio to payout for blackjack. Defaults to 3:2")]
    pub blackjack_payout: Ratio,

    /// Will the dealer hit on a soft 17?
    #[flag(
        description = "Will the dealer hit on a soft 17? The dealer will stand without this flag specified."
    )]
    pub hit_soft_17: bool,
}

/// The default maximum number of players that can connect
const DEFAULT_MAX_PLAYERS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };

/// The default timeout to reject clients if they haven't returned a hello message
const DEFAULT_CONNECTION_TIMEOUT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(300) };

/// The default maximum amount a player can bet for hand in a single round
const DEFAULT_MAX_BET: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(1000) };

/// The default minimum amount a player must bet for a hand in a single round
const DEFAULT_MIN_BET: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(10) };

/// The default minimum number of players to begin a round
const DEFUALT_MIN_PLAYERS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };

/// The default minimum number of human players to begin a round
const DEFUALT_MIN_HUMANS: u8 = 0;

/// The default payout for blackjacks
const DEFAULT_BLACKJACK_PAYOUT: Ratio = unsafe { Ratio::new_unchecked(3, 2) };

/// The default maximum number of hands a player can start a round withs
const DEFAULT_MAX_HANDS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

/// The default number of decks to play the shoe with
const DEFAULT_DECKS: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(6) };
