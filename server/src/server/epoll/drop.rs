use super::EPoll;
use linux::unistd::close;

impl Drop for EPoll {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
