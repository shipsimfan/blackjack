use connection::Connection;
use init::init;
use options::Options;
use run::run;

mod connection;
mod init;
mod options;
mod run;
mod virtual_terminal;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
