mod bool;
mod number;
mod string;

/// An element which can be generated for a message
pub trait Generate {
    /// Generate this element for a message into `output`
    fn generate(&self, output: &mut Vec<u8>);
}
