use crate::messages::Generate;

impl Generate for String {
    fn generate(&self, output: &mut Vec<u8>) {
        assert!(self.len() <= u8::MAX as usize);
        output.push(self.len() as u8);
        output.extend_from_slice(self.as_bytes());
    }
}
