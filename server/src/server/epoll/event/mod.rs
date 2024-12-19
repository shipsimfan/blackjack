mod get;

/// An event that occurs while waiting on an e-poll instance
#[repr(C, packed(4))]
pub struct EPollEvent {
    /// The events that occured on the handle
    events: u32,

    /// The id of the handle that generated this event, specified when the handle was registered
    id: u64,
}
