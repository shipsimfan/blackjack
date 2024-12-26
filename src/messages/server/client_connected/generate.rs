use crate::messages::{ClientConnectedServerMessage, Generate};

impl<'a> Generate for ClientConnectedServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            ClientConnectedServerMessage::Borrowed(borrowed) => borrowed.generate(output),
            ClientConnectedServerMessage::Owned(owned) => owned.generate(output),
        }
    }
}
