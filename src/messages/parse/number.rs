use crate::messages::{Parse, ParseMessageError, Parser};

macro_rules! number_parse {
    ($($ty: ty),*) => {$(
        impl Parse for $ty {
            fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
                let mut buffer = [0; std::mem::size_of::<Self>()];
                for i in 0..buffer.len() {
                    buffer[i] = parser.next_err()?;
                }

                Ok(Self::from_be_bytes(buffer))
            }
        }

        impl Parse for std::num::NonZero<$ty> {
            fn parse(parser: &mut Parser) -> Result<Self, ParseMessageError> {
                std::num::NonZero::new(Parse::parse(parser)?).ok_or(ParseMessageError)
            }
        }
    )*};
}

number_parse!(u8, u16, u32, u64, i8, i16, i32, i64);
