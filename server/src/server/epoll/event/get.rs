use super::EPollEvent;

impl EPollEvent {
    /// Gets the id of the handle that triggered this event
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Gets the events that occured on the handle
    pub fn events(&self) -> u32 {
        self.events
    }
}
