use crate::{messages::Generate, model::Ratio};

impl Generate for Ratio {
    fn generate(&self, output: &mut Vec<u8>) {
        self.numerator.generate(output);
        self.denominator.generate(output);
    }
}
