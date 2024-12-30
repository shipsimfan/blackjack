mod from_key_code;

/// A key which cannot be representd by a character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
    /// The left arrow key
    LeftArrow = 37,

    /// The right arrow key
    RightArrow = 39,

    /// The up arrow key
    UpArrow = 38,

    /// The down arrow key
    DownArrow = 40,

    /// The delete key
    Delete = 46,

    /// The home key
    Home = 36,

    /// The end key
    End = 35,
}
