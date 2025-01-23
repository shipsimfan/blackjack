use argparse::Command;
use std::num::NonZeroU16;

/// The options the user selected for the AI
#[derive(Command)]
#[command(version, help)]
pub struct Options {
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
