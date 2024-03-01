//! Common code for `Ipld` errors.
use crate::ipld::Ipld;

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
