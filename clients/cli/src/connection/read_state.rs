/// The current item being read by a client socket
pub(super) enum ReadState {
    /// The header is being read
    Header,

    /// The body is being read
    Body,
}
