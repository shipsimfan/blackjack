use crate::messages::Generate;

impl Generate for bool {
    fn generate(&self, output: &mut Vec<u8>) {
        output.push(*self as u8);
    }
}
