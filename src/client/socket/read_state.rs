/// The current item being read from the socket
pub(super) enum ReadState {
    /// The header is currently being read
    Header,

    /// The body is currently being read
    Body,
}
