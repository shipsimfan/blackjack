use argparse::Command;
use lobby::Lobby;
use options::Options;
use server::Server;

mod lobby;
mod options;
mod server;

fn main() {
    if let Err(error) = run() {
        eprintln!("[ERROR] Error while starting server: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = match Options::parse_env()? {
        Some(options) => options,
        None => return Ok(()),
    };

    let server = Server::new(options.addr(), options.max_players())?;
    let lobby = Lobby::new(options);

    println!(
        "[INFO] Listening on {} . . .",
        todo!("Get actually bound address from server")
    );

    server.run(lobby)
}
