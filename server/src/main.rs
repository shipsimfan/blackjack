use argparse::Command;
use options::Options;
use server::Server;

mod options;
mod server;

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

    let server = Server::new(options.addr(), options.max_players())?;

    println!("Listening on {} . . .", options.addr());

    Ok(())
}
