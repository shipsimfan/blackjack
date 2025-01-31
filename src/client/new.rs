use crate::{
    client::{Client, ClientError, Options, Socket, AI},
    messages::{HelloClientMessage, PlayNextRoundClientMessage, ServerMessage, Username, Version},
};
use argparse::Command;
use std::borrow::Cow;

impl<T: AI> Client<T> {
    /// Creates and runs a new [`Client`]
    pub fn new(
        client_name: &str,
        client_version: Version,
    ) -> Result<(), ClientError<T::CreationError>> {
        let options = match T::Options::parse_env().map_err(ClientError::ArgumentParseError)? {
            Some(options) => options,
            None => return Ok(()),
        };

        let username = Username::new(options.username()).ok_or(ClientError::InvalidUsername)?;

        let mut socket = Socket::connect(options.address(), options.port())?;

        let hello = match socket.read_message()? {
            Some(ServerMessage::Hello(hello)) => hello,
            Some(_) => return Err(ClientError::UnexpectedMessage),
            None => {
                println!("Disconnected from server");
                return Ok(());
            }
        };

        let password = if hello.password_required() {
            match options.password() {
                Some(password) => Some(Cow::Borrowed(password)),
                None => return Err(ClientError::NoPassword),
            }
        } else {
            None
        };

        socket.send_message(HelloClientMessage::new(
            username,
            password,
            client_name,
            client_version,
            true,
        ))?;

        socket.send_message(PlayNextRoundClientMessage::new(true))?;

        let ai = T::new(options).map_err(ClientError::CreationError)?;

        Client { ai, socket }.run()
    }
}
