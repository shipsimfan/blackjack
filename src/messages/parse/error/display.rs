use crate::messages::ParseMessageError;

impl std::fmt::Display for ParseMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("an invalid message was received")
    }
}
