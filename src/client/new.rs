use crate::client::{Client, ClientError, Options, AI};
use argparse::Command;
use std::net::TcpStream;

impl<T: AI> Client<T> {
    /// Creates and runs a new [`Client`]
    pub fn new() -> Result<(), ClientError<T::CreationError>> {
        let options = match T::Options::parse_env().map_err(ClientError::ArgumentParseError)? {
            Some(options) => options,
            None => return Ok(()),
        };

        let socket =
            TcpStream::connect((options.address(), options.port().get())).map_err(|error| {
                ClientError::ConnectError(error, options.address().to_owned(), options.port().get())
            })?;

        let ai = T::new(options).map_err(ClientError::CreationError)?;

        Client { ai, socket }.run()
    }
}
