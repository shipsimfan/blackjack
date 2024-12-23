use crate::Options;
use argparse::Command;

/// Initialize the elements for the program
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let options = match Options::parse_env()? {
        Some(options) => options,
        None => return Ok(()),
    };

    Ok(())
}
