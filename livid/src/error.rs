use std::fmt;
use std::io;
use std::string::FromUtf8Error;

/// Error types returned by livid + wrappers of std errors
#[derive(Debug)]
#[non_exhaustive]
pub enum LividError {
    /// i/o error
    IoError(io::Error),
    /// Utf-8 conversion error
    Utf8Error(FromUtf8Error),
    /// Null string conversion error
    NullError(std::ffi::NulError),
    /// Internal livid error
    Internal(LividErrorKind),
    /// Error using an erroneous env variable
    EnvVarError(std::env::VarError),
    /// Parsing error
    ParseIntError(std::num::ParseIntError),
    /// Unknown error
    Unknown(String),
}

unsafe impl Send for LividError {}
unsafe impl Sync for LividError {}

/// Error kinds enum for `LividError`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LividErrorKind {
    /// Failed to run the application
    FailedToRun,
    /// Failed to initialize the multithreading
    FailedToLock,
    /// Failed to set the general scheme of the application
    FailedToSetScheme,
    /// Failed operation, mostly unknown reason!
    FailedOperation,
    /// System resource (file, image) not found
    ResourceNotFound,
    /// Image format error when opening an image of an unsupported format
    ImageFormatError,
    /// Error filling table
    TableError,
    /// Error due to printing
    PrintError,
    /// Invalid color
    InvalidColor,
}

impl std::error::Error for LividError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LividError::IoError(err) => Some(err),
            LividError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for LividError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LividError::IoError(ref err) => err.fmt(f),
            LividError::NullError(ref err) => err.fmt(f),
            LividError::Internal(ref err) => write!(f, "An internal error occurred {:?}", err),
            LividError::EnvVarError(ref err) => write!(f, "An env var error occurred {:?}", err),
            LividError::Utf8Error(ref err) => {
                write!(f, "A UTF8 conversion error occurred {:?}", err)
            }
            LividError::ParseIntError(ref err) => {
                write!(f, "An int parsing error occurred {:?}", err)
            }
            LividError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for LividError {
    fn from(err: io::Error) -> LividError {
        LividError::IoError(err)
    }
}

impl From<std::ffi::NulError> for LividError {
    fn from(err: std::ffi::NulError) -> LividError {
        LividError::NullError(err)
    }
}

impl From<std::env::VarError> for LividError {
    fn from(err: std::env::VarError) -> LividError {
        LividError::EnvVarError(err)
    }
}

impl From<std::string::FromUtf8Error> for LividError {
    fn from(err: std::string::FromUtf8Error) -> LividError {
        LividError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for LividError {
    fn from(err: std::num::ParseIntError) -> LividError {
        LividError::ParseIntError(err)
    }
}
