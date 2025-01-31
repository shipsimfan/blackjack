use crate::client::{ClientError, Socket};
use std::{net::TcpStream, num::NonZeroU16};

impl Socket {
    /// Connect to `address`
    pub fn connect<E: std::error::Error>(
        address: &str,
        port: NonZeroU16,
    ) -> Result<Self, ClientError<E>> {
        let socket = TcpStream::connect((address, port.get()))
            .map_err(|error| ClientError::ConnectError(error, address.to_owned(), port.get()))?;

        Ok(Socket {
            socket,
            body_buffer: Vec::with_capacity(u16::MAX as usize),
            write_buffer: Vec::with_capacity(u16::MAX as usize),
        })
    }
}
