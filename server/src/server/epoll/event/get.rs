use super::EPollEvent;

impl EPollEvent {
    /// Gets the id of the handle that triggered this event
    pub fn id(&self) -> u64 {
        self.id
    }
}
