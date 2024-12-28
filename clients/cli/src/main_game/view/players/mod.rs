use player::PlayerView;

mod new;
mod player;
mod render;
mod truncate;

/// The view of the different players
pub struct PlayersView {
    /// The players
    players: Vec<PlayerView>,

    /// The y-level to start displaying players at
    y: usize,

    /// The width available to render in
    width: usize,

    /// The id of the local player
    local_id: usize,
}
