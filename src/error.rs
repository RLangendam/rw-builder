use thiserror::Error;

/// The error type for rw-builder operations.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Standard I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Missing stdout in child process
    #[error("Missing child stdout: {0}")]
    MissingChildStdout(&'static str),

    /// Missing stdin in child process
    #[error("Missing child stdin: {0}")]
    MissingChildStdin(&'static str),

    /// Error returned by wincode read operations
    #[cfg(feature = "wincode")]
    #[error("Wincode read error: {0}")]
    WincodeRead(#[from] wincode::ReadError),

    /// Error returned by wincode write operations
    #[cfg(feature = "wincode")]
    #[error("Wincode write error: {0}")]
    WincodeWrite(#[from] wincode::WriteError),

    /// Error returned by wincode IO write operations
    #[cfg(feature = "wincode")]
    #[error("Wincode IO write error: {0}")]
    WincodeIoWrite(#[from] wincode::io::WriteError),

    /// String from UTF-8 conversion error
    #[error("UTF-8 decoding error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

/// The result type for rw-builder operations.
pub type Result<T> = std::result::Result<T, Error>;
