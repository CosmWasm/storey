use std::{borrow::Borrow, marker::PhantomData};

use crate::storage::IterableStorage;
use crate::storage::StorageBranch;

use super::{KeyDecodeError, Storable, StorableIter};

pub struct Map<K: ?Sized, V> {
    prefix: &'static [u8],
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V> Map<K, V>
where
    K: OwnedKey,
    V: Storable,
{
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn access<S>(&self, storage: S) -> MapAccess<K, V, StorageBranch<S>> {
        Self::access_impl(StorageBranch::new(storage, self.prefix.to_vec()))
    }
}

impl<K, V> Storable for Map<K, V>
where
    K: OwnedKey,
    V: Storable,
{
    type AccessorT<S> = MapAccess<K, V, S>;
    type Key = (K, V::Key);
    type Value = V::Value;
    type ValueDecodeError = V::ValueDecodeError;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, S> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(key: &[u8]) -> Result<Self::Key, KeyDecodeError> {
        let len = *key.first().ok_or(KeyDecodeError)? as usize;

        if key.len() < len + 1 {
            return Err(KeyDecodeError);
        }

        let map_key = K::from_bytes(&key[1..len + 1]).map_err(|_| KeyDecodeError)?;
        let rest = V::decode_key(&key[len + 1..])?;

        Ok((map_key, rest))
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        V::decode_value(value)
    }
}

pub struct MapAccess<K: ?Sized, V, S> {
    storage: S,
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V, S> MapAccess<K, V, S>
where
    K: Key,
    V: Storable,
{
    pub fn entry<Q>(&self, key: &Q) -> V::AccessorT<StorageBranch<&S>>
    where
        K: Borrow<Q>,
        Q: Key + ?Sized,
    {
        let len = key.bytes().len();
        let bytes = key.bytes();
        let mut key = Vec::with_capacity(len + 1);

        key.push(len as u8);
        key.extend_from_slice(bytes);

        V::access_impl(StorageBranch::new(&self.storage, key))
    }

    pub fn entry_mut<Q>(&mut self, key: &Q) -> V::AccessorT<StorageBranch<&mut S>>
    where
        K: Borrow<Q>,
        Q: Key + ?Sized,
    {
        let len = key.bytes().len();
        let bytes = key.bytes();
        let mut key = Vec::with_capacity(len + 1);

        key.push(len as u8);
        key.extend_from_slice(bytes);

        V::access_impl(StorageBranch::new(&mut self.storage, key))
    }
}

impl<K, V, S> MapAccess<K, V, S>
where
    K: OwnedKey,
    V: Storable,
    S: IterableStorage,
{
    pub fn iter<'s>(
        &'s self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> StorableIter<'s, Map<K, V>, S> {
        StorableIter {
            inner: self.storage.pairs(start, end),
            phantom: PhantomData,
        }
    }
}

pub trait Key {
    fn bytes(&self) -> &[u8];
}

pub trait OwnedKey: Key {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ()>
    where
        Self: Sized;
}

impl Key for String {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl OwnedKey for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ()>
    where
        Self: Sized,
    {
        std::str::from_utf8(bytes).map(String::from).map_err(|_| ())
    }
}

impl Key for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
