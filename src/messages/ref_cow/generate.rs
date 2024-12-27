use crate::messages::{Generate, RefCow};

impl<'a, T: Generate> Generate for RefCow<'a, T> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            RefCow::Borrowed(borrowed) => *borrowed,
            RefCow::Owned(owned) => owned,
        }
        .generate(output);
    }
}
