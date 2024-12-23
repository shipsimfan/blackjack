use win32::HANDLE;

/// A representation of a virtual terminal
pub struct VirtualTerminal {
    /// The output handle for writing to the terminal
    output: HANDLE,

    /// The input handle for reading from the terminal
    input: HANDLE,
}
