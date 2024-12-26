use app_state::AppState;
use connection::{Connection, ConnectionError};
use options::Options;
use run::run;
use virtual_terminal::{
    SpecialKey, TerminalEvent, TextInput, VirtualTerminal, VirtualTerminalCreationError,
};

mod app_state;
mod connection;
mod options;
mod run;
mod virtual_terminal;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
