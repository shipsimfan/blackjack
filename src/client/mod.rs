//! A framework for implementing AI clients

mod ai;
mod r#move;
mod new;
mod run;

pub use ai::AI;
pub use r#move::Move;

/// A framework for implementing AI clients
pub struct Client<T: AI> {
    /// The ai running this client
    ai: T,
}
