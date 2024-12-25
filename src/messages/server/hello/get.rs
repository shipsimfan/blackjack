use crate::messages::{HelloServerMessage, Version};

impl<'a> HelloServerMessage<'a> {
    /// Does the server require a password?
    pub fn password_required(&self) -> bool {
        self.password_required
    }

    /// Gets the name of the server
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Gets the name of the application the server is running
    pub fn server_application_name(&self) -> &str {
        &self.server_application_name
    }

    /// Gets the version of the server application
    pub fn server_version(&self) -> Version {
        self.server_version
    }
}
