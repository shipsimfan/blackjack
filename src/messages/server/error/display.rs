use crate::messages::ErrorServerMessage;

impl std::fmt::Display for ErrorServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ErrorServerMessage::ServerFull => "server is full",
            ErrorServerMessage::ConnectionTimeout => "timed out during connection",
            ErrorServerMessage::InvalidPassword => "invalid password",
            ErrorServerMessage::UsernameTaken => "username taken",
        })
    }
}
