use super::ReadMessageError;

impl std::fmt::Display for ReadMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadMessageError::IO(error) => error.fmt(f),
            ReadMessageError::Parse(error) => error.fmt(f),
        }
    }
}
