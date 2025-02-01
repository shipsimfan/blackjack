use crate::{
    client::{Client, ClientError, AI},
    messages::PlaceBetClientMessage,
    model::GameState,
};

impl<T: AI> Client<T> {
    /// Run the client
    pub(super) fn run(mut self) -> Result<(), ClientError<T::CreationError>> {
        loop {
            match self.table.state() {
                GameState::WaitingForPlayers => {}
                GameState::WaitingForBets => {
                    if let Some(place_bet) = self.check_for_bet() {
                        self.socket.send_message(place_bet)?;
                    }
                }
                GameState::WaitingForPlayer(player, _) => {
                    if self.client_id == player {
                        todo!("Get action from ai");
                    }
                }
            }

            todo!("After taking an action, if needed, wait for a server message");
        }
    }

    /// Check if we have placed a bet, and place one if not
    fn check_for_bet(&mut self) -> Option<PlaceBetClientMessage> {
        todo!()
    }
}
