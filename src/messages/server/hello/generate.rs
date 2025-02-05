use crate::messages::{Generate, HelloServerMessage};

impl<'a> Generate for HelloServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.protocol_version.generate(output);
        self.password_required.generate(output);
        self.server_name.generate(output);
        self.server_application_name.generate(output);
        self.server_version.generate(output);
    }
}
