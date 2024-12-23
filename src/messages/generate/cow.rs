use crate::messages::Generate;
use std::borrow::Cow;

impl<'a, T: Generate + Clone> Generate for Cow<'a, T> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.as_ref().generate(output);
    }
}
