use crate::Options;
use std::net::{Ipv6Addr, SocketAddr};

impl Options {
    /// Gets the address to listen for incoming connections with
    pub fn addr(&self) -> SocketAddr {
        let ip = self.address.unwrap_or(Ipv6Addr::UNSPECIFIED.into());
        SocketAddr::new(ip, self.port.get())
    }

    /// Gets the maximum number of players that can connect at one time
    pub fn max_players(&self) -> usize {
        self.max_players.get() as _
    }
}
