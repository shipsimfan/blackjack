use super::RegisteredHandle;
use linux::unistd::close;

impl Drop for RegisteredHandle {
    fn drop(&mut self) {
        self.epoll.borrow_mut().unregister(self.handle).unwrap();
        unsafe { close(self.handle) };
    }
}
