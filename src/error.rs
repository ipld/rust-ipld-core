//! `Ipld` error definitions.
use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;
use core::fmt;

use crate::ipld::Ipld;

/// Error during Serde operations.
#[cfg(feature = "serde")]
#[derive(Clone, Debug)]
pub struct SerdeError(String);

#[cfg(feature = "serde")]
impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "serde error: {}", self.0)
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for SerdeError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::StdError for SerdeError {}

/// Error used for converting from and into [`crate::ipld::Ipld`].
#[derive(Clone, Debug)]
pub enum ConversionError {
    /// Error when the IPLD kind wasn't the one we expected.
    WrongIpldKind {
        /// The expected type.
        expected: KindErrorType,
        /// The actual type.
        found: KindErrorType,
    },
    /// Error when the given Ipld kind cannot be converted into a certain value type.
    FromIpld {
        /// The IPLD kind trying to convert from.
        from: KindErrorType,
        /// The type trying to convert into.
        into: &'static str,
    },
    /// Error when a certain map or list element cannot be accessed.
    Access(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongIpldKind { expected, found } => {
                write!(
                    formatter,
                    "kind error: expected {:?} but found {:?}",
                    expected, found
                )
            }
            Self::FromIpld { from, into } => {
                write!(
                    formatter,
                    "conversion error: cannot convert {:?} into {}",
                    from, into
                )
            }
            Self::Access(error) => {
                write!(formatter, "access error: {}", error)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionError {}

/// Error types for the IPLD kinds.
///
/// It maps the IPLD kinds into a unity-only enum.
#[derive(Clone, Debug)]
pub enum KindErrorType {
    /// Null type.
    Null,
    /// Boolean type.
    Bool,
    /// Integer type.
    Integer,
    /// Float type.
    Float,
    /// String type.
    String,
    /// Bytes type.
    Bytes,
    /// List type.
    List,
    /// Map type.
    Map,
    /// Link type.
    Link,
}

impl From<Ipld> for KindErrorType {
    fn from(ipld: Ipld) -> Self {
        Self::from(&ipld)
    }
}

impl From<&Ipld> for KindErrorType {
    fn from(ipld: &Ipld) -> Self {
        match ipld {
            Ipld::Null => Self::Null,
            Ipld::Bool(_) => Self::Bool,
            Ipld::Integer(_) => Self::Integer,
            Ipld::Float(_) => Self::Float,
            Ipld::String(_) => Self::String,
            Ipld::Bytes(_) => Self::Bytes,
            Ipld::List(_) => Self::List,
            Ipld::Map(_) => Self::Map,
            Ipld::Link(_) => Self::Link,
        }
    }
}

/// Error when accessing IPLD List or Map elements.
#[derive(Clone, Debug)]
pub struct AccessError(String);

impl AccessError {
    /// Create a new error.
    pub fn new(error: String) -> Self {
        Self(error)
    }
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "access error: {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AccessError {}
