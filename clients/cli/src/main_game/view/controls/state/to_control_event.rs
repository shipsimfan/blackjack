use super::ControlState;
use crate::main_game::view::controls::ControlEvent;
use std::num::NonZeroU16;

impl ControlState {
    /// Get the corresponding control event for `option`
    pub fn to_control_event(&self, option: usize) -> Option<ControlEvent> {
        match self {
            ControlState::None => None,
            ControlState::PlayNextRound => Some(ControlEvent::PlayNextRound),
            ControlState::DontPlayNextRound => Some(ControlEvent::DontPlayNextRound),
            ControlState::Betting => Some(ControlEvent::PlaceBet(
                NonZeroU16::new(option as _).unwrap(),
            )),
            ControlState::HitStand => Some(match option {
                0 => ControlEvent::Hit,
                _ => ControlEvent::Stand,
            }),
        }
    }
}
