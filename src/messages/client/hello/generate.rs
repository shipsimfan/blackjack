use crate::messages::{Generate, HelloClientMessage};

impl Generate for HelloClientMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.username.generate(output);
        self.password.generate(output);
        self.client_name.generate(output);
        self.client_version.generate(output);
        self.ai.generate(output);
    }
}
