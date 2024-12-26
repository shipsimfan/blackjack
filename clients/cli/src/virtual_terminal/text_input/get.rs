use crate::TextInput;

impl TextInput {
    /// Gets the currently entered value in the input
    pub fn value(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.value) }
    }
}
