use crate::{
    client::{socket::ReadState, ClientError, Socket},
    messages::header::HEADER_SIZE,
};
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
            read_state: ReadState::Header,
            header_buffer: vec![0; HEADER_SIZE].into_boxed_slice(),
            body_buffer: Vec::with_capacity(u16::MAX as usize),
            last_tag: 0,
            read_length: 0,
            write_buffer: Vec::with_capacity(u16::MAX as usize),
        })
    }
}
