use crate::messages::{Generate, Username};

impl Generate for Username {
    fn generate(&self, output: &mut Vec<u8>) {
        self.inner.generate(output);
    }
}
