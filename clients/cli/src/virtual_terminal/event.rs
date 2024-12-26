use crate::SpecialKey;

/// The result of the virtual terminal processing an event
pub enum TerminalEvent {
    /// The event does not require any further processing
    Ignored,

    /// The program should exit because of this event
    Exit,

    /// The terminal was re-sized, re-rendering might be required
    Resize,

    /// A character was entered
    Character(char),

    /// A special key was pressed
    SpecialKey(SpecialKey),
}
