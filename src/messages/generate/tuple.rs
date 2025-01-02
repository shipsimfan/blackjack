use crate::messages::Generate;

impl<T1: Generate, T2: Generate> Generate for (T1, T2) {
    fn generate(&self, output: &mut Vec<u8>) {
        self.0.generate(output);
        self.1.generate(output);
    }
}
