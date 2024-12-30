use super::ControlState;
use blackjack::model::{BlackjackTable, PlayerState};

impl ControlState {
    /// Get the current control state based on `table`
    pub fn get(table: &BlackjackTable, local_id: usize) -> Self {
        let player = table.player(local_id);

        match player.state() {
            PlayerState::NotPlaying => ControlState::PlayNextRound,
            PlayerState::PlayingNextRound => ControlState::DontPlayNextRound,
            PlayerState::PlayingThisRound => ControlState::None,
        }
    }
}
