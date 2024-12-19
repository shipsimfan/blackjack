use super::Handle;
use linux::unistd::close;

impl Drop for Handle {
    fn drop(&mut self) {
        match self {
            Handle::Unregistered(handle) => {
                unsafe { close(*handle) };
            }
            Handle::Registered(_) => {}
        }
    }
}
