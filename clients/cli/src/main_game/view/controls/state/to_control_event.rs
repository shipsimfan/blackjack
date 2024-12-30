use super::ControlState;
use crate::main_game::view::controls::ControlEvent;

impl ControlState {
    /// Get the corresponding control event for `option`
    pub fn to_control_event(&self, option: usize) -> Option<ControlEvent> {
        match self {
            ControlState::None => None,
            ControlState::PlayNextRound => Some(ControlEvent::PlayNextRound),
            ControlState::DontPlayNextRound => Some(ControlEvent::DontPlayNextRound),
            ControlState::Betting => todo!(),
        }
    }
}
