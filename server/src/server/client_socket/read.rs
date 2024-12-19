use super::ClientSocket;
use blackjack::messages::ClientMessage;

impl ClientSocket {
    /// Continues reading the next message from this socket
    pub fn read(&mut self) -> linux::Result<Option<ClientMessage>> {
        todo!()
    }
}
