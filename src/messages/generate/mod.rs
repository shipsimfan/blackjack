mod bool;
mod collections;
mod cow;
mod number;
mod string;
mod tuple;

/// An element which can be generated for a message
pub(crate) trait Generate {
    /// Generate this element for a message into `output`
    fn generate(&self, output: &mut Vec<u8>);
}
