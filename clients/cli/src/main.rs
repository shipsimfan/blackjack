use connection::{Connection, ConnectionError};
use game_state::GameState;
use options::Options;
use run::run;
use virtual_terminal::{VirtualTerminal, VirtualTerminalCreationError};

mod connection;
mod game_state;
mod options;
mod run;
mod virtual_terminal;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
