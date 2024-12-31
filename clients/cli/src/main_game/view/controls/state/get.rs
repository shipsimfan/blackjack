use super::ControlState;
use blackjack::model::{BlackjackTable, PlayerState};

impl ControlState {
    /// Get the current control state based on `table`
    pub fn get(table: &BlackjackTable, local_id: usize) -> Self {
        let player = table.player(local_id);

        match player.state() {
            PlayerState::NotPlaying => return ControlState::PlayNextRound,
            PlayerState::PlayingNextRound => {
                return if table.state().is_round_active() {
                    ControlState::DontPlayNextRound
                } else {
                    ControlState::Betting
                }
            }
            PlayerState::PlayingThisRound => {
                if !table.state().is_round_active() || table.current_player().unwrap() != local_id {
                    return ControlState::None;
                }
            }
        };

        todo!("Handle input during play")
    }
}
