use super::{ClientWriter, Handle};
use blackjack::messages::ServerMessage;
use linux::{
    fcntl::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK},
    try_linux,
    unistd::write,
};

fn do_write(buffer: &mut [u8], handle: &Handle) -> linux::Result<()> {
    // Disable non-blocking
    let mut flags = try_linux!(fcntl(**handle, F_GETFL, 0))?;
    flags &= !O_NONBLOCK;
    try_linux!(fcntl(**handle, F_SETFL, flags))?;

    // Perform write
    let mut written = 0;
    while written < buffer.len() {
        written += try_linux!(write(
            **handle,
            buffer[written..].as_ptr().cast(),
            (buffer.len() - written) as _
        ))? as usize;
    }

    // Re-enable non-blocking
    flags |= O_NONBLOCK;
    try_linux!(fcntl(**handle, F_SETFL, flags))?;
    Ok(())
}

impl ClientWriter {
    /// Send a message to this client
    pub fn send(&mut self, message: &ServerMessage) {
        let handle_ref = self.handle.borrow_mut();
        let handle = match &*handle_ref {
            Some(handle) => handle,
            None => return,
        };

        message.generate(&mut self.write_buffer);

        if let Err(error) = do_write(&mut self.write_buffer, handle) {
            drop(handle_ref);
            eprintln!(
                "[ERROR] Error while writing a message to a client: {}",
                error
            );
            self.disconnect();
        }
    }
}
