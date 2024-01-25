//! IPLD core
//!
//! This crate provides core types for interoperating with [IPLD].
//!
//! [IPLD]: https://ipld.io/
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod ipld;
#[cfg(feature = "serde")]
pub mod serde;
