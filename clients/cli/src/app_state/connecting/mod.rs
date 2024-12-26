use blackjack::messages::Username;

mod handle_message;
mod new;
mod unwrap;

/// Currently connecting to the server
pub struct Connecting {
    /// The username to present as
    username: Username<'static>,
}
