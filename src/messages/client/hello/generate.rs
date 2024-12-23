use crate::messages::{Generate, HelloClientMessage};

impl<'a> Generate for HelloClientMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.username.generate(output);
        self.password.generate(output);
        self.client_name.generate(output);
        self.client_version.generate(output);
        self.ai.generate(output);
    }
}
