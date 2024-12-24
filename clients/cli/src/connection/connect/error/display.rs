use crate::ConnectionError;

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unable to connect to {}:{} - {}",
            self.address.0, self.address.1, self.error
        )
    }
}
