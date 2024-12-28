mod clear;
mod get;
mod new;
mod render;

/// A view of a single player
pub struct PlayerView {
    /// The username of the player
    username: String,

    /// Is this player an AI?
    ai: bool,

    /// Is this player the local player?
    is_local: bool,

    /// The y-level to render this player at
    y: usize,

    /// The width available to render
    width: usize,
}
