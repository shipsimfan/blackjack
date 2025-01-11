mod get;
mod options;
mod to_control_event;

/// The current state of the controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlState {
    /// No controls to display
    None,

    /// Display the controls to play the next round
    PlayNextRound,

    /// Display the controls to not play the next round
    DontPlayNextRound,

    /// Display the betting input
    Betting,

    /// Display the option to hit or stand
    HitStand,
}
