//! `Ipld` error definitions.
use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;

use crate::ipld::{Ipld, IpldIndex};
pub use anyhow::{Error, Result};
#[cfg(feature = "std")]

/// Error during Serde operations.
#[cfg(feature = "serde")]
#[derive(Clone, Debug)]
pub struct SerdeError(String);

#[cfg(feature = "serde")]
impl core::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Serde error: {}", self.0)
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
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::StdError for SerdeError {}

/// Type error.
#[derive(Clone, Debug)]
pub struct TypeError {
    /// The expected type.
    pub expected: TypeErrorType,
    /// The actual type.
    pub found: TypeErrorType,
}

impl TypeError {
    /// Creates a new type error.
    pub fn new<A: Into<TypeErrorType>, B: Into<TypeErrorType>>(expected: A, found: B) -> Self {
        Self {
            expected: expected.into(),
            found: found.into(),
        }
    }
}

impl core::fmt::Display for TypeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Expected {:?} but found {:?}", self.expected, self.found)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TypeError {}

/// Type error type.
#[derive(Clone, Debug)]
pub enum TypeErrorType {
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
    /// Key type.
    Key(String),
    /// Index type.
    Index(usize),
}

impl From<Ipld> for TypeErrorType {
    fn from(ipld: Ipld) -> Self {
        Self::from(&ipld)
    }
}

impl From<&Ipld> for TypeErrorType {
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

impl From<IpldIndex<'_>> for TypeErrorType {
    fn from(index: IpldIndex<'_>) -> Self {
        match index {
            IpldIndex::List(i) => Self::Index(i),
            IpldIndex::Map(s) => Self::Key(s),
            IpldIndex::MapRef(s) => Self::Key(s.into()),
        }
    }
}
