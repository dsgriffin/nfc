use crate::ffi;
use std::error;
use std::ffi::CStr;
use std::fmt;

/// The result type used throughout this crate's safe API.
pub type Result<T> = std::result::Result<T, Error>;

/// An error reported by libnfc or by this crate's safe wrapper layer.
///
/// Errors usually come from a negative libnfc return code, but the safe API
/// also uses this type for wrapper-level failures such as invalid UTF-8
/// assumptions or malformed user input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    code: Option<i32>,
    message: String,
}

impl Error {
    /// Creates an error with a message but no libnfc error code.
    ///
    /// This is mainly used for wrapper-layer validation failures.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
        }
    }

    /// Creates an error from a libnfc device pointer and error code.
    pub(crate) fn from_device(device: *mut ffi::nfc_device, code: i32) -> Self {
        let message = if device.is_null() {
            format!("libnfc error code {code}")
        } else {
            unsafe { CStr::from_ptr(ffi::nfc_strerror(device)).to_string_lossy().into_owned() }
        };

        Self {
            code: Some(code),
            message,
        }
    }

    /// Returns the libnfc error code when one is available.
    ///
    /// Wrapper-generated errors return `None`.
    pub fn code(&self) -> Option<i32> {
        self.code
    }

    /// Returns the human-readable error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "{} ({code})", self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl error::Error for Error {}
