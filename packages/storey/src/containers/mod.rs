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
    type Kind: StorableKind;

    /// The accessor type for this collection/container. An accessor is a type that provides
    /// methods for reading and writing to the collection/container and encapsulates the
    /// specific [`Storage`] type used (the `S` type parameter here).
    ///
    /// [`Storage`]: crate::storage::Storage
    type Accessor<S>;

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
    fn access_impl<S>(storage: S) -> Self::Accessor<S>;

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

/// A trait for collection accessors (see [`Storable::Accessor`]) that provide iteration over
/// their contents.
pub trait IterableAccessor: Sized {
    /// The [`Storable`] type this accessor is associated with.
    type Storable: Storable;

    /// The [`Storage`] type this accessor is associated with.
    ///
    /// [`Storage`]: crate::storage::Storage
    type Storage: IterableStorage;

    /// Get a reference to the storage this accessor is associated with.
    fn storage(&self) -> &Self::Storage;

    /// Iterate over key-value pairs in this collection.
    fn pairs(&self) -> StorableIter<'_, Self::Storable, Self::Storage> {
        StorableIter {
            inner: self.storage().pairs(None, None),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection.
    fn keys(&self) -> StorableKeys<'_, Self::Storable, Self::Storage> {
        StorableKeys {
            inner: self.storage().keys(None, None),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection.
    fn values(&self) -> StorableValues<'_, Self::Storable, Self::Storage> {
        StorableValues {
            inner: self.storage().values(None, None),
            phantom: PhantomData,
        }
    }
}

pub trait BoundedIterableAccessor: IterableAccessor {
    /// Iterate over key-value pairs in this collection, respecting the given bounds.
    fn bounded_pairs<S, E>(
        &self,
        start: Option<S>,
        end: Option<E>,
    ) -> StorableIter<'_, Self::Storable, Self::Storage>
    where
        S: BoundFor<Self::Storable>,
        E: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableIter {
            inner: self.storage().pairs(start.as_deref(), end.as_deref()),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection, respecting the given bounds.
    fn bounded_keys<S, E>(
        &self,
        start: Option<S>,
        end: Option<E>,
    ) -> StorableKeys<'_, Self::Storable, Self::Storage>
    where
        S: BoundFor<Self::Storable>,
        E: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableKeys {
            inner: self.storage().keys(start.as_deref(), end.as_deref()),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection, respecting the given bounds.
    fn bounded_values<S, E>(
        &self,
        start: Option<S>,
        end: Option<E>,
    ) -> StorableValues<'_, Self::Storable, Self::Storage>
    where
        S: BoundFor<Self::Storable>,
        E: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableValues {
            inner: self.storage().values(start.as_deref(), end.as_deref()),
            phantom: PhantomData,
        }
    }
}

/// A type that can be used as bounds for iteration over a given collection.
///
/// As an example, a collection `Foo` with string-y keys can accept both `String` and
/// `&str` bounds by providing these impls:
/// - `impl BoundFor<Foo> for &str`
/// - `impl BoundFor<Foo> for String`
pub trait BoundFor<T> {
    fn into_bytes(self) -> Vec<u8>;
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

/// The kind of a storable.
///
/// This is used to differentiate between terminal and non-terminal storables.
/// See also: [`Terminal`] and [`NonTerminal`].
///
/// This trait is [sealed](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
/// and cannot be implemented outside of this crate.
pub trait StorableKind {}

/// A terminal [`Storable`] kind. A terminal storable doesn't manage any subkeys,
/// and is the end of the line in a composable collection.
///
/// An example of a terminal storable is [`Item`], but not [`Column`] or [`Map`].
pub struct Terminal;

/// A non-terminal [`Storable`] kind. A non-terminal storable manages subkeys.
///
/// Some examples of non-terminal storables are [`Column`] and [`Map`].
pub struct NonTerminal;

impl StorableKind for Terminal {}
impl StorableKind for NonTerminal {}
