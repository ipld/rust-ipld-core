use alloc::string::{String, ToString};
use core::fmt;

#[derive(Clone, Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Serde error: {}", self.0)
    }
}

//#[cfg(feature = "std")]
//impl std::error::Error for Error {}
//
impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: core::fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl serde::ser::StdError for Error {}
