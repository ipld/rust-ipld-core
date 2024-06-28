//! This module contains traits to have a unified API across codecs.
//!
//! There are two traits defined, [`Codec`] and [`Links`]. Those are separate traits as the `Links`
//! trait is not generic over a certain type.

use cid::Cid;

use std::io::{BufRead, Write};

/// Each IPLD codec implementation should implement this Codec trait. This way codecs can be more
/// easily exchanged or combined.
pub trait Codec<T>: Links {
    /// The error that is returned if encoding or decoding fails.
    type Error;

    /// The multicodec code of the IPLD codec.
    fn to_code(&self) -> u64;
    /// Attempt to convert from a `u64` code to this `Codec`.
    fn try_from_code(code: u64) -> Option<Self> where Self: Sized;

    /// Decode a reader into the desired type.
    fn decode<R: BufRead>(&self, reader: R) -> Result<T, Self::Error>;
    /// Encode a type into a writer.
    fn encode<W: Write>(&self, writer: W, data: &T) -> Result<(), Self::Error>;

    /// Decode a slice into the desired type.
    fn decode_from_slice(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        self.decode(bytes)
    }

    /// Encode a type into bytes.
    fn encode_to_vec(&self, data: &T) -> Result<Vec<u8>, Self::Error> {
        let mut output = Vec::new();
        self.encode(&mut output, data)?;
        Ok(output)
    }
}

/// Trait for returning the links of a serialized IPLD data.
pub trait Links {
    /// The error that is returned if the link extraction fails.
    type LinksError;

    /// Return all links (CIDs) that the given encoded data contains.
    fn links(&self, bytes: &[u8]) -> Result<impl Iterator<Item = Cid>, Self::LinksError>;
}
