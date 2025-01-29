use crate::client::{Client, ClientError, Options, Socket, AI};
use argparse::Command;
use std::net::TcpStream;

impl<T: AI> Client<T> {
    /// Creates and runs a new [`Client`]
    pub fn new() -> Result<(), ClientError<T::CreationError>> {
        let options = match T::Options::parse_env().map_err(ClientError::ArgumentParseError)? {
            Some(options) => options,
            None => return Ok(()),
        };

        let socket = Socket::connect(options.address(), options.port())?;

        let ai = T::new(options).map_err(ClientError::CreationError)?;

        Client { ai, socket }.run()
    }
}
