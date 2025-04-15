//! This module contains both the traits for implementing collections/containers, as well as a
//! few fundamental collections/containers themselves.

mod column;
pub mod common;
mod item;
pub mod map;
#[cfg(test)]
mod test_utils;

use std::{marker::PhantomData, ops::Bound};

use storey_storage::RevIterableStorage;

use crate::storage::IterableStorage;

pub use storey_macros::router;

pub use column::{Column, ColumnAccess};
pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

/// The fundamental trait every collection/container should implement.
pub trait Storable {
    type Kind: StorableKind;

    /// The accessor type for this collection/container. An accessor is a type that provides
    /// methods for reading and writing to the collection/container and encapsulates the
    /// specific [`Storage`] type used (the `S` type parameter here).
    ///
    /// [`Storage`]: crate::storage::Storage
    type Accessor<S>;

    /// Create an accessor for this collection/container, given a [`Storage`] implementation.
    ///
    /// [`Storage`]: crate::storage::Storage
    fn access_impl<S>(storage: S) -> Self::Accessor<S>;
}

pub trait IterableStorable: Storable {
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
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum KVDecodeError<K, V> {
    #[error("failed to decode key: {0}")]
    Key(K),
    #[error("failed to decode value: {0}")]
    Value(V),
}

impl<K: std::fmt::Display, V: std::fmt::Display> crate::error::StoreyError for KVDecodeError<K, V> {}

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
    fn pairs(
        &self,
    ) -> StorableIter<Self::Storable, <Self::Storage as IterableStorage>::PairsIterator<'_>> {
        StorableIter {
            inner: self.storage().pairs(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection.
    fn keys(
        &self,
    ) -> StorableKeys<Self::Storable, <Self::Storage as IterableStorage>::KeysIterator<'_>> {
        StorableKeys {
            inner: self.storage().keys(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection.
    fn values(
        &self,
    ) -> StorableValues<Self::Storable, <Self::Storage as IterableStorage>::ValuesIterator<'_>>
    {
        StorableValues {
            inner: self.storage().values(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }
}

pub trait RevIterableAccessor
where
    Self: IterableAccessor,
    Self::Storage: RevIterableStorage,
{
    /// Iterate over key-value pairs in this collection in reverse order.
    fn rev_pairs(
        &self,
    ) -> StorableIter<Self::Storable, <Self::Storage as RevIterableStorage>::RevPairsIterator<'_>>
    {
        StorableIter {
            inner: self.storage().rev_pairs(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection in reverse order.
    fn rev_keys(
        &self,
    ) -> StorableKeys<Self::Storable, <Self::Storage as RevIterableStorage>::RevKeysIterator<'_>>
    {
        StorableKeys {
            inner: self.storage().rev_keys(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection in reverse order.
    fn rev_values(
        &self,
    ) -> StorableValues<Self::Storable, <Self::Storage as RevIterableStorage>::RevValuesIterator<'_>>
    {
        StorableValues {
            inner: self
                .storage()
                .rev_values(Bound::Unbounded, Bound::Unbounded),
            phantom: PhantomData,
        }
    }
}

impl<I> RevIterableAccessor for I
where
    I: IterableAccessor,
    I::Storage: RevIterableStorage,
{
}

/// This trait extends [`IterableAccessor`] with methods for bounded iteration. Not every
/// iterable collection supports it, so this trait is separate.
///
/// Bounded iteration allows the user to specify a start and end bound for the iteration.
///
/// # Why not always support bounded iteration?
///
/// The reason bounded iteration isn't always supported is that it may not be possible to
/// efficiently implement it. For example, a `Map<String, Map<u8, _>` must length-prefix
/// the string keys when mapping them to the byte keys used in the storage backend. Otherwise,
/// we'd not know where the string key ends and the nested `u8` key begins.
///
/// The length prefix will interfere with the ordering of the keys so that it's no longer
/// lexicographical. It's deterministic, but rather confusing and unlikely to be useful. This
/// in turn means the entries found between two string keys may not be the expected ones.
pub trait BoundedIterableAccessor: IterableAccessor {
    /// Iterate over key-value pairs in this collection, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_pairs<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableIter<Self::Storable, <Self::Storage as IterableStorage>::PairsIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableIter {
            inner: self.storage().pairs(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_keys<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableKeys<Self::Storable, <Self::Storage as IterableStorage>::KeysIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableKeys {
            inner: self.storage().keys(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_values<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableValues<Self::Storable, <Self::Storage as IterableStorage>::ValuesIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableValues {
            inner: self.storage().values(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }
}

/// This trait extends [`BoundedIterableAccessor`] with methods for bounded reverse iteration.
/// Not every iterable collection supports it, so this trait is separate.
///
/// Bounded reverse iteration allows the user to specify a start and end bound for the iteration,
/// but in reverse order.
///
/// # Why not always support bounded reverse iteration?
///
/// The same reasons as for [bounded iteration](BoundedIterableAccessor) apply.
pub trait BoundedRevIterableAccessor
where
    Self: BoundedIterableAccessor,
    Self::Storage: RevIterableStorage,
{
    /// Iterate over key-value pairs in this collection in reverse order, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_rev_pairs<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableIter<Self::Storable, <Self::Storage as RevIterableStorage>::RevPairsIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableIter {
            inner: self.storage().rev_pairs(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }

    /// Iterate over keys in this collection in reverse order, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_rev_keys<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableKeys<Self::Storable, <Self::Storage as RevIterableStorage>::RevKeysIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableKeys {
            inner: self.storage().rev_keys(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }

    /// Iterate over values in this collection in reverse order, respecting the given bounds.
    ///
    /// Either end of the range can be unbounded, inclusive, or exclusive. See [`Bound`] for more.
    fn bounded_rev_values<B>(
        &self,
        start: Bound<B>,
        end: Bound<B>,
    ) -> StorableValues<Self::Storable, <Self::Storage as RevIterableStorage>::RevValuesIterator<'_>>
    where
        B: BoundFor<Self::Storable>,
    {
        let start = start.map(|b| b.into_bytes());
        let end = end.map(|b| b.into_bytes());

        StorableValues {
            inner: self.storage().rev_values(
                start.as_ref().map(|b| b.as_slice()),
                end.as_ref().map(|b| b.as_slice()),
            ),
            phantom: PhantomData,
        }
    }
}

impl<I> BoundedRevIterableAccessor for I
where
    I: BoundedIterableAccessor,
    I::Storage: RevIterableStorage,
{
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
pub struct StorableIter<S, I> {
    inner: I,
    phantom: PhantomData<S>,
}

impl<S, I> Iterator for StorableIter<S, I>
where
    S: IterableStorable,
    I: Iterator<Item = (Vec<u8>, Vec<u8>)>,
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
pub struct StorableKeys<S, I> {
    inner: I,
    phantom: PhantomData<S>,
}

impl<S, I> Iterator for StorableKeys<S, I>
where
    S: IterableStorable,
    I: Iterator<Item = Vec<u8>>,
{
    type Item = Result<S::Key, S::KeyDecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|k| S::decode_key(&k))
    }
}

/// The iterator over values in a collection.
pub struct StorableValues<S, I> {
    inner: I,
    phantom: PhantomData<S>,
}

impl<S, I> Iterator for StorableValues<S, I>
where
    S: IterableStorable,
    I: Iterator<Item = Vec<u8>>,
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
pub trait StorableKind: sealed::StorableKindSeal {}

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

mod sealed {
    pub trait StorableKindSeal {}

    impl StorableKindSeal for super::Terminal {}
    impl StorableKindSeal for super::NonTerminal {}
}
