use crate::SpecialKey;
use win32::WORD;

impl SpecialKey {
    /// Convert `key_code` in to a special key
    pub(in crate::virtual_terminal) fn from_key_code(key_code: WORD) -> Option<Self> {
        Some(match key_code {
            35 => SpecialKey::End,
            36 => SpecialKey::Home,
            37 => SpecialKey::LeftArrow,
            38 => SpecialKey::UpArrow,
            39 => SpecialKey::RightArrow,
            40 => SpecialKey::DownArrow,
            46 => SpecialKey::Delete,
            _ => return None,
        })
    }
}
