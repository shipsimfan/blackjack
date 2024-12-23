macro_rules! messages {
    (
        $(#[$meta: meta])*
        [client = $client: literal]
        $vis: vis enum $ident: ident<$lifetime: lifetime> {$(
            $(#[$inner_meta: meta])*
            $name: ident($struct: ident$(<$variant_lifetime: lifetime>)*) = $tag: literal,
        )*}
) => {
        $(#[$meta])*
        $vis enum $ident<$lifetime> {$(
            $(#[$inner_meta])*
            $name($struct$(<$variant_lifetime>)*),
        )*}

        impl<$lifetime> $ident<$lifetime> {
            /// Parse `content` into the message identified by `tag`
            pub fn parse(tag: u16, content: &[u8]) -> Result<Self, $crate::messages::ParseMessageError> {
                let mut parser = $crate::messages::Parser::new(content);

                let ret = match tag {$(
                    $tag => $ident::$name($crate::messages::Parse::parse(&mut parser)?),
                )*
                    _ => return Err($crate::messages::ParseMessageError),
                };

                if !parser.empty() {
                    return Err($crate::messages::ParseMessageError);
                }

                Ok(ret)
            }

            /// Gets the tag that identifies this message
            pub fn tag(&self) -> u16 {
                match self {$(
                    $ident::$name(_) => $tag,
                )*}
            }

            /// Generate the message into `output`, including header
            pub fn generate(&self, output: &mut Vec<u8>) {
                output.truncate(0);
                output.extend_from_slice(&[0; $crate::messages::header::HEADER_SIZE]);

                match self {$(
                    $ident::$name(message) => $crate::messages::Generate::generate(message, output),
                )*}

                $crate::messages::header::generate(output, $client, self.tag());
            }
        }
    };
}

pub(super) use messages;
