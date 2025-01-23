#![feature(random)]

use std::num::NonZeroU16;

use argparse::Command;
use blackjack::client::Client;
use options::Options;

mod ai;
mod options;

struct RandomAI {
    /// The value which must be generated less than or equal to to hit
    hit_chance: Option<u64>,

    /// The amount to bet each round
    bet: NonZeroU16,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = match Options::parse_env()? {
        Some(options) => options,
        None => return Ok(()),
    };

    Client::new(RandomAI {
        hit_chance: options
            .hit_chance
            .map(|chance| (chance * (u32::MAX + 1) as f32) as u64),
        bet: options.bet,
    })
    .run()?;

    Ok(())
}
