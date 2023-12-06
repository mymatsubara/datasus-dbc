#[derive(Debug)]
pub enum Error {
    /// An IO error
    Io(std::io::Error),
    /// Error while decompressing the content of the file
    Decompression(explode::Error),
    /// File without dbc header
    MissingHeader,
    /// Header size is greater than the file size
    InvalidHeaderSize,
}

/// Result type from reading a dbc file
pub type Result<T> = std::result::Result<T, Error>;

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::Decompression(err) => write!(f, "{}", err),
            Error::MissingHeader => write!(f, "file does not contain dbc header or is empty"),
            Error::InvalidHeaderSize => write!(f, "dbc header size is greater than the file size"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Decompression(err) => Some(err),
            _ => None,
        }
    }
}
