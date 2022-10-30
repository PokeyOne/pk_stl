/// The main error type for this crate.
///
/// Provides indication of binary or ascii and the message.
#[derive(Debug, Clone)]
pub struct Error {
    /// True if the error was in a binary file, false if it was in an ascii
    /// file.
    binary: bool,
    /// The error message.
    message: String
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new error that occurred in a binary file.
    pub fn binary(msg: &str) -> Error {
        Error {
            binary: true,
            message: msg.to_string()
        }
    }

    /// Create a new error that occurred in an ASCII file.
    pub fn ascii(msg: &str) -> Error {
        Error {
            binary: false,
            message: msg.to_string()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bin_or_ascii_str = if self.binary { "Binary" } else { "ASCII" };
        write!(f, "{} STL Parse Error: {}", bin_or_ascii_str, self.message)
    }
}

impl std::error::Error for Error {}