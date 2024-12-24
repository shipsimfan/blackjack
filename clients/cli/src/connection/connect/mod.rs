use super::ReadState;
use crate::Connection;
use blackjack::messages::header::HEADER_SIZE;
use socket_address::SocketAddress;
use std::{
    net::ToSocketAddrs,
    ptr::{null, null_mut},
};
use win32::{
    try_get_last_error, try_wsa_get_last_error,
    winsock2::{closesocket, connect, socket, INVALID_SOCKET, SOCK_STREAM},
    CloseHandle, CreateEvent, FALSE, OVERLAPPED,
};

mod error;
mod socket_address;

pub use error::ConnectionError;

impl Connection {
    /// Connect to `address:port`
    pub fn connect(address: &str, port: u16) -> Result<Self, ConnectionError> {
        let socket_addr = SocketAddress::from(
            (address, port)
                .to_socket_addrs()
                .map_err(|error| ConnectionError::new(address, port, error))?
                .next()
                .ok_or_else(|| ConnectionError::new(address, port, "no address found"))?,
        );

        let handle = unsafe { socket(socket_addr.family(), SOCK_STREAM, 0) };
        if handle == INVALID_SOCKET {
            return Err(ConnectionError::new(
                address,
                port,
                win32::Error::wsa_get_last_error(),
            ));
        }

        let event = try_get_last_error!(CreateEvent(null_mut(), FALSE, FALSE, null())).map_err(
            |error| {
                unsafe { closesocket(handle) };
                ConnectionError::new(address, port, error)
            },
        )?;

        try_wsa_get_last_error!(connect(
            handle,
            socket_addr.as_ptr(),
            socket_addr.len() as _
        ))
        .map_err(|error| {
            unsafe {
                closesocket(handle);
                CloseHandle(event);
            }
            ConnectionError::new(address, port, error)
        })?;

        let overlapped = Box::new(OVERLAPPED {
            internal: 0,
            internal_high: 0,
            union: win32::OVERLAPPED_UNION {
                pointer: null_mut(),
            },
            event,
        });

        let mut connection = Connection {
            handle,
            event,
            overlapped,
            read_state: ReadState::Header,
            header_buffer: vec![0; HEADER_SIZE].into_boxed_slice(),
            body_buffer: Vec::with_capacity(u16::MAX as usize),
            last_tag: 0,
            read_length: 0,
        };
        connection
            .begin_read()
            .map_err(|error| ConnectionError::new(address, port, error))?;
        Ok(connection)
    }
}
