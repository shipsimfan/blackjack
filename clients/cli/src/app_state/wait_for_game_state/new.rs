use super::WaitForGameState;
use crate::Connection;
use blackjack::{
    messages::{HelloClientMessage, Username},
    pkg_version,
};
use std::borrow::Cow;

impl WaitForGameState {
    /// Creates a new [`WaitForGameState`]
    pub fn new(
        connection: &mut Connection,
        username: Username,
        password: Option<&str>,
        server_name: String,
    ) -> Self {
        connection.send(HelloClientMessage::new(
            username,
            password.map(Cow::Borrowed),
            env!("CARGO_PKG_NAME"),
            pkg_version!(),
            false,
        ));

        WaitForGameState { server_name }
    }
}
