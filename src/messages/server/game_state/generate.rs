use crate::messages::{GameStateServerMessage, Generate};

impl<'a> Generate for GameStateServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            GameStateServerMessage::Borrowed(borrowed) => *borrowed,
            GameStateServerMessage::Owned(owned) => owned,
        }
        .generate(output);
    }
}
