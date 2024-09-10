//! An integration of [`storey`] with [*CosmWasm*].
//!
//! This crate provides
//! - a [*CosmWasm*] storage backend for use with [`storey`] collections,
//! - a [*MessagePack*] encoding integration to be used for serializing and deserializing
//!   values, and
//! - a set of container re-exports that remove the need to manually specify the
//!   encoding, instead relying on the default [*MessagePack*] encoding.
//!
//! [*CosmWasm*]: https://github.com/CosmWasm/cosmwasm
//! [*MessagePack*]: https://msgpack.org/

mod backend;
pub mod containers;
mod encoding;

pub use backend::CwStorage;
pub use encoding::CwEncoding;
