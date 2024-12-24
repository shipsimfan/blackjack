use crate::ConnectionError;

impl ConnectionError {
    pub fn new<E: std::fmt::Display>(address: &str, port: u16, error: E) -> Self {
        ConnectionError {
            address: (address.to_owned(), port),
            error: error.to_string(),
        }
    }
}
