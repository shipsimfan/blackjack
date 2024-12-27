use crate::messages::Generate;
use std::borrow::Cow;

impl Generate for str {
    fn generate(&self, output: &mut Vec<u8>) {
        assert!(self.len() <= u8::MAX as usize && self.len() > 0);
        output.push(self.len() as u8);
        output.extend_from_slice(self.as_bytes());
    }
}

impl Generate for String {
    fn generate(&self, output: &mut Vec<u8>) {
        self.as_str().generate(output);
    }
}

impl Generate for Option<String> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            Some(str) => str.generate(output),
            None => 0u8.generate(output),
        }
    }
}

impl Generate for Option<&str> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            Some(str) => str.generate(output),
            None => 0u8.generate(output),
        }
    }
}

impl<'a> Generate for Option<Cow<'a, str>> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            Some(str) => str.generate(output),
            None => 0u8.generate(output),
        }
    }
}
