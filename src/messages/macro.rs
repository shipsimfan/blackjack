macro_rules! messages {
    (
        $(#[$meta: meta])*
        $vis: vis enum $ident: ident {$(
            $(#[$inner_meta: meta])*
            $name: ident($struct: ident) = $tag: literal,
        )*}
) => {
        $(#[$meta])*
        $vis enum $ident {$(
            $(#[$inner_meta])*
            $name($struct),
        )*}

        impl $ident {
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
        }
    };
}

pub(super) use messages;
