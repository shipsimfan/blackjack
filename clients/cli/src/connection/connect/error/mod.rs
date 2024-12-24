mod display;
mod new;

/// An error that occured while connecting to the server
#[derive(Debug)]
pub struct ConnectionError {
    /// The address the connection was attempted to
    address: (String, u16),

    /// The error preventing connection
    error: String,
}

impl std::error::Error for ConnectionError {}
