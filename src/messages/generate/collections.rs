use crate::messages::Generate;

macro_rules! list_generate {
    ($($ty: ty),*) => {$(
        impl<T: Generate> Generate for $ty {
            fn generate(&self, output: &mut Vec<u8>) {
                assert!(self.len() <= u8::MAX as usize);
                (self.len() as u8).generate(output);
                for item in self {
                    item.generate(output);
                }
            }
        }
    )*};
}

list_generate!(Vec<T>, Box<[T]>);
