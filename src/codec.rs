//! This module contains traits to have a unified API across codecs.
//!
//! There are two traits defined, [`Codec`] and [`Links`]. Those are separate traits as the `Links`
//! trait is not generic over a certain type.

use cid::Cid;

/// Each IPLD codec implementation should implement this Codec trait. This way codecs can be more
/// easily exchanged or combined.
pub trait Codec<T>: Links {
    /// The multicodec code of the IPLD codec.
    const CODE: u64;
    /// The error that is returned if encoding or decoding fails.
    type Error;

    /// Decode a slice into the desired type.
    fn decode(bytes: &[u8]) -> Result<T, Self::Error>;
    /// Encode a type into bytes.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error>;
}

/// Trait for returning the links of a serialized IPLD data.
pub trait Links {
    /// The error that is returned if the link extraction fails.
    type LinksError;

    /// Return all links (CIDs) that the given encoded data contains.
    fn links(bytes: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinksError>;
}
