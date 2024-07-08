use std::{borrow::Borrow, marker::PhantomData};

use crate::storage::IterableStorage;
use crate::storage::StorageBranch;

use super::IterableAccessor;
use super::Storable;

/// A map that stores values of type `V` under keys of type `K`.
///
/// The subkeys managed by the map are length-prefixed and appended to the map's prefix.
///
/// A map does not directly manage the storage of its values. Instead, it doles out access to
/// a collection of other containers.
///
/// # Examples
///
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::{Item, Map};
///
/// let mut storage = TestStorage::new();
/// let map = Map::<String, Item<u64, TestEncoding>>::new(0);
/// let mut access = map.access(&mut storage);
///
/// access.entry_mut("foo").set(&1337).unwrap();
/// assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
/// assert_eq!(access.entry("bar").get().unwrap(), None);
/// ```
///
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::{Item, Map};
///
/// let mut storage = TestStorage::new();
/// let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(0);
/// let mut access = map.access(&mut storage);
///
/// access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
/// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), Some(1337));
/// assert_eq!(access.entry("foo").entry("baz").get().unwrap(), None);
/// ```
pub struct Map<K: ?Sized, V> {
    prefix: u8,
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V> Map<K, V>
where
    K: OwnedKey,
    V: Storable,
    <V as Storable>::KeyDecodeError: std::fmt::Display,
{
    /// Creates a new map with the given prefix.
    ///
    /// It is the responsibility of the caller to ensure that the prefix is unique and does not conflict
    /// with other keys in the storage.
    ///
    /// The key provided here is used as a prefix for all keys managed by the map.
    pub const fn new(prefix: u8) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    /// Acquires an accessor for the map.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map};
    ///
    /// // immutable access
    /// let storage = TestStorage::new();
    /// let map = Map::<String, Item<u64, TestEncoding>>::new(0);
    /// let access = map.access(&storage);
    ///
    /// // mutable access
    /// let mut storage = TestStorage::new();
    /// let map = Map::<String, Item<u64, TestEncoding>>::new(0);
    /// let mut access = map.access(&mut storage);
    /// ```
    pub fn access<S>(&self, storage: S) -> MapAccess<K, V, StorageBranch<S>> {
        Self::access_impl(StorageBranch::new(storage, vec![self.prefix]))
    }
}

impl<K, V> Storable for Map<K, V>
where
    K: OwnedKey,
    V: Storable,
    <V as Storable>::KeyDecodeError: std::fmt::Display,
{
    type Accessor<S> = MapAccess<K, V, S>;
    type Key = (K, V::Key);
    type KeyDecodeError = MapKeyDecodeError<V::KeyDecodeError>;
    type Value = V::Value;
    type ValueDecodeError = V::ValueDecodeError;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, S> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(key: &[u8]) -> Result<Self::Key, MapKeyDecodeError<V::KeyDecodeError>> {
        let len = *key.first().ok_or(MapKeyDecodeError::EmptyKey)? as usize;

        if key.len() < len + 1 {
            return Err(MapKeyDecodeError::KeyTooShort(len));
        }

        let map_key =
            K::from_bytes(&key[1..len + 1]).map_err(|_| MapKeyDecodeError::InvalidUtf8)?;
        let rest = V::decode_key(&key[len + 1..]).map_err(MapKeyDecodeError::Inner)?;

        Ok((map_key, rest))
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        V::decode_value(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid key length, expected empty key")]
pub enum MapKeyDecodeError<I: std::fmt::Display> {
    #[error("empty key, expected length prefix (1 byte)")]
    EmptyKey,

    #[error("key too short, expected {0} bytes after length prefix")]
    KeyTooShort(usize),

    #[error("invalid UTF8")]
    InvalidUtf8,

    #[error("sub key decode error: {0}")]
    Inner(I),
}

/// An accessor for a map.
///
/// The accessor provides methods for interacting with the map in storage.
pub struct MapAccess<K: ?Sized, V, S> {
    storage: S,
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V, S> MapAccess<K, V, S>
where
    K: Key,
    V: Storable,
{
    /// Returns an immutable accessor for the inner container of this map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map};
    ///
    /// let storage = TestStorage::new();
    /// let map = Map::<String, Item<u64, TestEncoding>>::new(0);
    /// let access = map.access(&storage);
    ///
    /// assert_eq!(access.entry("foo").get().unwrap(), None);
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map};
    ///
    /// let storage = TestStorage::new();
    /// let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(0);
    /// let access = map.access(&storage);
    ///
    /// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), None);
    /// ```
    pub fn entry<Q>(&self, key: &Q) -> V::Accessor<StorageBranch<&S>>
    where
        K: Borrow<Q>,
        Q: Key + ?Sized,
    {
        let key = length_prefixed_key(key);

        V::access_impl(StorageBranch::new(&self.storage, key))
    }

    /// Returns a mutable accessor for the inner container of this map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map};
    ///
    /// let mut storage = TestStorage::new();
    /// let map = Map::<String, Item<u64, TestEncoding>>::new(0);
    /// let mut access = map.access(&mut storage);
    ///
    /// access.entry_mut("foo").set(&1337).unwrap();
    /// assert_eq!(access.entry("foo").get().unwrap(), Some(1337));
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, Map};
    ///
    /// let mut storage = TestStorage::new();
    /// let map = Map::<String, Map<String, Item<u64, TestEncoding>>>::new(0);
    /// let mut access = map.access(&mut storage);
    ///
    /// access.entry_mut("foo").entry_mut("bar").set(&1337).unwrap();
    /// assert_eq!(access.entry("foo").entry("bar").get().unwrap(), Some(1337));
    /// ```
    pub fn entry_mut<Q>(&mut self, key: &Q) -> V::Accessor<StorageBranch<&mut S>>
    where
        K: Borrow<Q>,
        Q: Key + ?Sized,
    {
        let key = length_prefixed_key(key);

        V::access_impl(StorageBranch::new(&mut self.storage, key))
    }
}

fn length_prefixed_key<K: Key + ?Sized>(key: &K) -> Vec<u8> {
    let len = key.bytes().len();
    let bytes = key.bytes();
    let mut key = Vec::with_capacity(len + 1);

    key.push(len as u8);
    key.extend_from_slice(bytes);

    key
}

impl<K, V, S> IterableAccessor for MapAccess<K, V, S>
where
    K: OwnedKey,
    V: Storable,
    <V as Storable>::KeyDecodeError: std::fmt::Display,
    S: IterableStorage,
{
    type Storable = Map<K, V>;
    type Storage = S;

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }
}

pub trait Key {
    fn bytes(&self) -> &[u8];
}

pub trait OwnedKey: Key {
    type Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Key for String {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Key for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid UTF8")]
pub struct InvalidUtf8;

impl OwnedKey for String {
    type Error = InvalidUtf8;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        std::str::from_utf8(bytes)
            .map(String::from)
            .map_err(|_| InvalidUtf8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::containers::Item;

    use mocks::backend::TestStorage;
    use mocks::encoding::TestEncoding;
    use storey_storage::Storage as _;

    #[test]
    fn map() {
        let mut storage = TestStorage::new();

        let map = Map::<String, Item<u64, TestEncoding>>::new(0);

        map.access(&mut storage)
            .entry_mut("foo")
            .set(&1337)
            .unwrap();

        assert_eq!(map.access(&storage).entry("foo").get().unwrap(), Some(1337));
        assert_eq!(
            storage.get(&[0, 3, 102, 111, 111]),
            Some(1337u64.to_le_bytes().to_vec())
        );
        map.access(&mut storage).entry_mut("foo").remove();

        assert_eq!(map.access(&storage).entry("foo").get().unwrap(), None);
        assert_eq!(map.access(&storage).entry("bar").get().unwrap(), None);
    }

    #[test]
    fn pairs() {
        let mut storage = TestStorage::new();

        let map = Map::<String, Item<u64, TestEncoding>>::new(0);
        let mut access = map.access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let items = access.pairs().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(
            items,
            vec![
                (("bar".to_string(), ()), 42),
                (("foo".to_string(), ()), 1337)
            ]
        );
    }

    #[test]
    fn keys() {
        let mut storage = TestStorage::new();

        let map = Map::<String, Item<u64, TestEncoding>>::new(0);
        let mut access = map.access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let keys = access.keys().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(keys, vec![("bar".to_string(), ()), ("foo".to_string(), ())])
    }

    #[test]
    fn values() {
        let mut storage = TestStorage::new();

        let map = Map::<String, Item<u64, TestEncoding>>::new(0);
        let mut access = map.access(&mut storage);

        access.entry_mut("foo").set(&1337).unwrap();
        access.entry_mut("bar").set(&42).unwrap();

        let values = access.values().collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(values, vec![42, 1337])
    }
}
