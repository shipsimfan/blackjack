use crate::server::ClientWriter;
use std::time::SystemTime;

mod disconnect;
mod get;
mod new;
mod send;
mod unwrap;

/// A client which hasn't sent a hello yet
pub struct ConnectingClient {
    /// The writer for the new client
    writer: ClientWriter,

    /// The time when the connection timesout
    timeout: SystemTime,
}
