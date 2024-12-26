use crate::messages::Username;

impl<'a> Username<'a> {
    /// Borrow this username
    pub fn borrow<'b>(&'b self) -> Username<'b> {
        Username::new(self.inner.as_ref()).unwrap()
    }
}
