use crate::init;

/// Run the program
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    init()?;

    Ok(())
}
