use chat::ChatView;
use players::PlayersView;

mod chat;
mod players;

mod add_message;
mod full_render;
mod new;
mod render;
mod resize;

pub struct View {
    /// The view of the chat messages
    chat: ChatView,

    /// The view of the players
    players: PlayersView,

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
const HAND_LINE_MARGIN: usize = 3; // [ ][ ][ ]

/// The space required to display the count of a given hand
const HAND_LINE_COUNT_LENGTH: usize = 4; // [HS ][0-9][0-9 ][ ]

/// The size of the largest possible hand
const LARGEST_HAND: usize = 21; // 20 aces + any other card

/// Extra characters all bets while have
const HAND_LINE_BET_EXTRA: usize = 2; // [$] and trailing [ ]

/// The minimum size that must be supported for a hand line
const MIN_HAND_LINE_LENGTH: usize =
    HAND_LINE_MARGIN + HAND_LINE_COUNT_LENGTH + LARGEST_HAND + HAND_LINE_BET_EXTRA;

/// The width of the vertical bar between the game and the chat
const VERTICAL_BAR_WIDTH: usize = 3; // [ ][|][ ]
