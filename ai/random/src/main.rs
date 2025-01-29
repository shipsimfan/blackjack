#![feature(random)]

use blackjack::client::Client;
use options::Options;
use std::num::NonZeroU16;

mod ai;
mod options;

struct RandomAI {
    /// The value which must be generated less than or equal to to hit
    hit_chance: Option<u64>,

    /// The amount to bet each round
    bet: NonZeroU16,
}

fn main() {
    if let Err(error) = Client::<RandomAI>::new() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
