use crate::TextInput;

impl TextInput {
    /// Gets the currently entered value in the input
    pub fn value(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.value) }
    }

    /// Gets the margin for the text input
    pub fn margin(&self) -> usize {
        self.margin
    }

    /// Gets the height of the text input
    pub fn height(&self) -> usize {
        self.height
    }
}
