use crate::model::GameState;

impl GameState {
    /// Is there currently an active round?
    pub fn is_round_active(&self) -> bool {
        match self {
            GameState::WaitingForBets | GameState::WaitingForPlayers => false,
            GameState::WaitingForPlayer(_, _) => true,
        }
    }
}
