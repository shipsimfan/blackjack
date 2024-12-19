use super::{EPoll, EPollEvent};
use linux::{sys::epoll::epoll_wait, try_linux};
use std::time::Duration;

impl EPoll {
    /// Waits for at least one event from the handles registered with this e-poll instance.
    ///
    /// If the result times-out, the `events` will be empty
    pub fn wait(
        &mut self,
        timeout: Option<Duration>,
        events: &mut Vec<EPollEvent>,
    ) -> linux::Result<()> {
        unsafe { events.set_len(0) };
        events.reserve(self.count);

        let event_count = try_linux!(epoll_wait(
            self.handle,
            events.as_mut_ptr().cast(),
            self.count as _,
            timeout
                .map(|duration| duration.as_millis() as i32)
                .unwrap_or(-1)
        ))?;

        unsafe { events.set_len(event_count as _) };

        Ok(())
    }
}
