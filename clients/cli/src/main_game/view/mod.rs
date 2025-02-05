use blackjack::messages::Username;
use chat::ChatView;
use controls::ControlsView;
use hand::HandView;
use players::PlayersView;

mod chat;
mod controls;
mod hand;
mod players;
mod view_event;

mod add_message;
mod full_render;
mod handle_terminal;
mod new;
mod render;
mod resize;

pub use view_event::ViewEvent;

pub struct View {
    /// The view of the chat messages
    chat: ChatView,

    /// The view of the dealer's hand
    dealer: HandView,

    /// The view of the players
    players: PlayersView,

    /// The view of the controls
    controls: ControlsView,

    /// The width of the game half
    game_width: usize,

    /// The name of the server
    server_name: String,

    /// The height the server name takes up
    server_name_height: usize,

    /// The y-level where the dealer's hand sits
    dealer_hand_y: usize,

    /// The minimum allowed bet
    min_bet: u16,

    /// The maximum allowed bet
    max_bet: u16,
}

/// The size of the margin before a hand line
const HAND_LINE_MARGIN: usize = "  ".len();

/// The space required to display the count of a given hand
const HAND_LINE_COUNT_LENGTH: usize = "H21 ".len(); // [HS ][0-9][0-9 ][ ]

/// The size of the largest possible hand
const LARGEST_HAND: usize = 21; // 20 aces + any other card

/// Extra characters all bets while have
const HAND_LINE_BET_EXTRA: usize = "$ ".len(); // [$] and trailing [ ]

/// The minimum size that must be supported for a hand line
const MIN_HAND_LINE_LENGTH: usize =
    HAND_LINE_MARGIN + HAND_LINE_COUNT_LENGTH + LARGEST_HAND + HAND_LINE_BET_EXTRA;

/// The width of the vertical bar between the game and the chat
const VERTICAL_BAR_WIDTH: usize = " | ".len();

/// The height reserved at the bottom of the terminal for the controls
const CONTROLS_HEIGHT: usize = 2;

/// The max amount of width a hand number will take up in "waiting for player"
const HAND_NUMBER_WIDTH: usize = " (255)".len();

/// The amount of extra space needed for the "waiting for player" status message
const WAITING_FOR_PLAYER_WIDTH: usize = "Waiting for player ...".len();

/// The width of the longest possible status message
const MIN_STATUS_LEN: usize = Username::MAX_LENGTH + HAND_NUMBER_WIDTH + WAITING_FOR_PLAYER_WIDTH;
