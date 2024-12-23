use crate::messages::ErrorServerMessage;

impl std::fmt::Display for ErrorServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ErrorServerMessage::ServerFull => "server is full",
        })
    }
}