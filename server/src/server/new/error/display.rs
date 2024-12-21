use super::NewServerError;

impl std::fmt::Display for NewServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewServerError::EPollCreationFailed(error) => {
                write!(f, "unable to create an e-poll instance - {}", error)
            }
            NewServerError::ListenSocketCreationFailed(error) => {
                write!(f, "unable to create the listen socket - {}", error)
            }
            NewServerError::BindSocketFailed(error, addr) => {
                write!(f, "unabled to bind listen socket to {} - {}", addr, error)
            }
            NewServerError::ListenSocketFailed(error) => {
                write!(f, "unable to begin listening for clients - {}", error)
            }
            NewServerError::SetNonBlockingFailed(error) => {
                write!(
                    f,
                    "unable to set the listen socket to non-blocking - {}",
                    error
                )
            }
            NewServerError::GetLocalAddressFailed(error) => write!(
                f,
                "unable to get the local address of the listen socket - {}",
                error
            ),
            NewServerError::RegisterListenSocketFailed(error) => write!(
                f,
                "unable to register the listen socket with the e-poll instance - {}",
                error
            ),
        }
    }
}
