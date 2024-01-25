IPLD core
=========

[![Crates.io](https://img.shields.io/crates/v/ipld-core.svg)](https://crates.io/crates/ipld-core)
[![Documentation](https://docs.rs/ipld-core/badge.svg)](https://docs.rs/ipld-core)

This crate provides core types for interoperating with [IPLD]. Codecs are not part of this crate, they are independent, but rely on `ipld-core`.

It can be seen as the successor of [`libipld-core`] with a smaller scope.

[IPLD]: https://ipld.io/
[`libipld-core`]: https://crates.io/crates/libipld-core


Features
--------

 - `std`: Makes the error message implement `std::error::Error`.
 - `serde`: Enables support for Serde serialization into/deserialization from the `Ipld` enum.


License
-------

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
