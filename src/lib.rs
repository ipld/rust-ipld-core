//! IPLD core
//!
//! This crate provides core types for interoperating with [IPLD].
//!
//! [IPLD]: https://ipld.io/
#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod convert;
pub mod error;
pub mod ipld;
#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "arb")]
mod arb;

pub use cid;
