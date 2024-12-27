use blackjack::model::BlackjackTable;
use view::View;

mod view;

mod handle_message;
mod handle_terminal;
mod new;

/// The main game state
pub struct MainGame {
    /// The id of this player
    client_id: u8,

    /// The model of the current state of the game
    table: BlackjackTable,

    /// The rendered view of the game
    view: View,
}
