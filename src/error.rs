#[derive(Debug, Clone)]
pub struct Error {
    binary: bool,
    message: String
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn binary(msg: &str) -> Error {
        Error {
            binary: true,
            message: msg.to_string()
        }
    }

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