use crate::messages::{Generate, Version};

impl Generate for Version {
    fn generate(&self, output: &mut Vec<u8>) {
        self.major.generate(output);
        self.minor.generate(output);
        self.patch.generate(output);
    }
}
