use super::ControlState;

impl ControlState {
    /// Gets the options to be presented in this control state
    pub fn options(&self) -> &[&'static str] {
        match self {
            ControlState::PlayNextRound => &["Play next round"],
            ControlState::DontPlayNextRound => &["Don't play next round"],
            ControlState::None | ControlState::Betting => &[],
        }
    }
}
