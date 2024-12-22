use crate::messages::{DisconnectClientMessage, Generate};

impl Generate for DisconnectClientMessage {
    fn generate(&self, _: &mut Vec<u8>) {}
}
