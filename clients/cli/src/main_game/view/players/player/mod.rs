use crate::main_game::view::hand::HandView;

mod clear;
mod get;
mod new;
mod render;

/// A view of a single player
pub struct PlayerView {
    /// The username of the player
    username: String,

    /// The currently displayed hands
    hands: Vec<HandView>,

    /// Is this player an AI?
    ai: bool,

    /// Is this player the local player?
    is_local: bool,

    /// Is the player not playing?
    not_playing: bool,

    /// The y-level to render this player at
    y: usize,

    /// The width available to render
    width: usize,
}
