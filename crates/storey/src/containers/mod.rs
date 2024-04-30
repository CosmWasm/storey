//! This module contains both the traits for implementing collections/containers, as well as a
//! few fundamental collections/containers themselves.

mod column;
mod item;
mod map;

use std::marker::PhantomData;

pub use column::{Column, ColumnAccess};
pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

use crate::storage::IterableStorage;

/// The fundamental trait every collection/container should implement.
pub trait Storable {
    /// The accessor type for this collection/container. An accessor is a type that provides
    /// methods for reading and writing to the collection/container and encapsulates the
    /// specific [`Storage`] type used (the `S` type parameter here).
    ///
    /// [`Storage`]: crate::storage::Storage
    type AccessorT<S>;

    /// The Key type for this collection/container. This is the type that will be used in
    /// key iteration.
    ///
    /// For composable collections this is the "full" key, e.g. for [`Map`]
    /// this is a tuple of the key and the sub-key.
    ///
    /// Containers that store one item and don't manage subkeys should use the `()` type here.
    type Key;

    /// The error type for decoding keys.
    type KeyDecodeError;

    /// The Value type for this collection/container. This is the type that will be used for
    /// value iteration.
    type Value;

    /// The error type for decoding values.
    type ValueDecodeError;

    /// Create an accessor for this collection/container, given a [`Storage`] implementation.
    ///
    /// [`Storage`]: crate::storage::Storage
    fn access_impl<S>(storage: S) -> Self::AccessorT<S>;

    /// Decode a key from a byte slice.
    ///
    /// This method is used in key iteration to provide a typed key rather than raw bytes
    /// to the user.
    fn decode_key(key: &[u8]) -> Result<Self::Key, Self::KeyDecodeError>;

    /// Decode a value from a byte slice.
    ///
    /// This method is used in value iteration to provide a typed value rather than raw bytes
    /// to the user.
    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError>;
}

/// A key-value pair decoding error.
#[derive(Debug, PartialEq)]
pub enum KVDecodeError<K, V> {
    Key(K),
    Value(V),
}

/// A trait for collection accessors (see [`Storable::AccessorT`]) that provide iteration over
/// their contents.
pub trait IterableAccessor {
    /// The [`Storable`] type this accessor is associated with.
    type StorableT: Storable;

    /// The [`Storage`] type this accessor is associated with.
    ///
    /// [`Storage`]: crate::storage::Storage
    type StorageT: IterableStorage;

    /// Get a reference to the storage this accessor is associated with.
    fn storage(&self) -> &Self::StorageT;

    /// Iterate over key-value pairs in this collection.
    fn pairs<'s>(
        &'s self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> StorableIter<'s, Self::StorableT, Self::StorageT> {
        StorableIter {
            inner: self.storage().pairs(start, end),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection.
    fn keys<'s>(
        &'s self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> StorableKeys<'s, Self::StorableT, Self::StorageT> {
        StorableKeys {
            inner: self.storage().keys(start, end),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection.
    fn values<'s>(
        &'s self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> StorableValues<'s, Self::StorableT, Self::StorageT> {
        StorableValues {
            inner: self.storage().values(start, end),
            phantom: PhantomData,
        }
    }
}

/// The iterator over key-value pairs in a collection.
pub struct StorableIter<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    inner: B::PairsIterator<'i>,
    phantom: PhantomData<S>,
}

impl<'i, S, B> Iterator for StorableIter<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    type Item = Result<(S::Key, S::Value), KVDecodeError<S::KeyDecodeError, S::ValueDecodeError>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| -> Self::Item {
            match (S::decode_key(&k), S::decode_value(&v)) {
                (Err(e), _) => Err(KVDecodeError::Key(e)),
                (_, Err(e)) => Err(KVDecodeError::Value(e)),
                (Ok(k), Ok(v)) => Ok((k, v)),
            }
        })
    }
}

/// The iterator over keys in a collection.
pub struct StorableKeys<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    inner: B::KeysIterator<'i>,
    phantom: PhantomData<S>,
}

impl<'i, S, B> Iterator for StorableKeys<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    type Item = Result<S::Key, S::KeyDecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|k| S::decode_key(&k))
    }
}

/// The iterator over values in a collection.
pub struct StorableValues<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    inner: B::ValuesIterator<'i>,
    phantom: PhantomData<S>,
}

impl<'i, S, B> Iterator for StorableValues<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    type Item = Result<S::Value, S::ValueDecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| S::decode_value(&v))
    }
}
