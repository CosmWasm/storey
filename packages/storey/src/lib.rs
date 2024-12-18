//! `storey` is an abstraction layer for blockchain storage backends.
//!
//! Typically, blockchain storage backends are binary key-value stores. `storey` abstracts over
//! these stores, providing
//! - a typed (rather than binary) interface,
//! - composable collections, and
//! - traits simplifying the implementation of new collections/containers.
//!
//! The encoding of keys is the responsibility of this framework and its collections.
//! The encoding of values is abstracted away by the traits in the [`encoding`] module.
//! Specific value encodings are implemented outside of this crate. It's not hard
//! to plug in any encoding you like.
//!
//! Similarly, the storage backend is pluggable. The [`storage`] module provides traits
//! for that.

pub mod containers;
pub mod encoding;
pub mod error;
pub mod storage;

pub fn foo() {
    println!("Hello, world!");
}
