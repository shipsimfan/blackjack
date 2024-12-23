use crate::messages::{Generate, Username};

impl<'a> Generate for Username<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.inner.generate(output);
    }
}
