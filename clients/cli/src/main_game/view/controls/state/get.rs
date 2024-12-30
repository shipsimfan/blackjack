use super::ControlState;
use blackjack::model::{BlackjackTable, GameState, PlayerState};

impl ControlState {
    /// Get the current control state based on `table`
    pub fn get(table: &BlackjackTable, local_id: usize) -> Self {
        let player = table.player(local_id);

        if table.state() == GameState::WaitingForPlayers
            || table.state() == GameState::WaitingForBets
        {
            return match player.state() {
                PlayerState::NotPlaying => ControlState::PlayNextRound,
                PlayerState::PlayingNextRound => ControlState::Betting,
                PlayerState::PlayingThisRound => ControlState::None,
            };
        }

        match player.state() {
            PlayerState::NotPlaying => return ControlState::PlayNextRound,
            PlayerState::PlayingNextRound => return ControlState::DontPlayNextRound,
            PlayerState::PlayingThisRound => {}
        }

        todo!("Handle input during play")
    }
}
