use crate::messages::Generate;

macro_rules! number_generate {
    ($($ty: ty),*) => {$(
        impl Generate for $ty {
            fn generate(&self, output: &mut Vec<u8>) {
                output.extend_from_slice(&self.to_be_bytes());
            }
        }

        impl Generate for std::num::NonZero<$ty> {
            fn generate(&self, output: &mut Vec<u8>) {
                self.get().generate(output);
            }
        }
    )*};
}

number_generate!(u8, u16, u32, u64, i8, i16, i32, i64);
