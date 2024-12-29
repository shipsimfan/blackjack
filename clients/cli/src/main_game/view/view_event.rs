/// An event that can happen from the view handling a terminal event
#[derive(Debug)]
pub enum ViewEvent {
    /// A chat message that should be sent
    Chat(String),
}
