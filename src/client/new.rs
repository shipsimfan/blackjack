use crate::{
    client::{Client, ClientError, Options, Socket, AI},
    messages::ServerMessage,
};
use argparse::Command;

impl<T: AI> Client<T> {
    /// Creates and runs a new [`Client`]
    pub fn new() -> Result<(), ClientError<T::CreationError>> {
        let options = match T::Options::parse_env().map_err(ClientError::ArgumentParseError)? {
            Some(options) => options,
            None => return Ok(()),
        };

        let mut socket = Socket::connect(options.address(), options.port())?;

        let hello = match socket.read_message()? {
            Some(ServerMessage::Hello(hello)) => hello,
            Some(_) => return Err(ClientError::UnexpectedMessage),
            None => {
                println!("Disconnected from server");
                return Ok(());
            }
        };

        let ai = T::new(options).map_err(ClientError::CreationError)?;

        Client { ai, socket }.run()
    }
}
