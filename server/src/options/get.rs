use crate::Options;
use std::{
    net::{Ipv6Addr, SocketAddr},
    time::Duration,
};

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

    /// Gets the number of seconds to wait before timing out a connecting client
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout.get() as _)
    }
}
