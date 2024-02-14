use std::{borrow::Borrow, marker::PhantomData};

use crate::storage_branch::StorageBranch;
use crate::{IterableStorage, Storage};

use super::Storable;

pub struct Map<K: ?Sized, V> {
    prefix: &'static [u8],
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V> Map<K, V>
where
    V: Storable,
{
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn access<'s, S: Storage + 's>(
        &self,
        storage: &'s S,
    ) -> MapAccess<K, V, StorageBranch<'s, S>> {
        Self::access_impl(StorageBranch::new(storage, self.prefix.to_vec()))
    }
}

impl<K, V> Storable for Map<K, V>
where
    V: Storable,
{
    type AccessorT<S> = MapAccess<K, V, S>;
    type Key = (K, V::Key);
    type KeyDecodeError = ();
    type Value = V::Value;
    type ValueDecodeError = V::ValueDecodeError;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, S> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(_key: &[u8]) -> Result<Self::Key, ()> {
        todo!()
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
    S: Storage,
{
    pub fn get<'s, Q>(&'s self, key: &Q) -> V::AccessorT<StorageBranch<'s, S>>
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
}

impl<K, V, S> MapAccess<K, V, S>
where
    K: Key,
    V: Storable,
    S: IterableStorage,
{
    pub fn iter<'s>(&'s self, start: Option<&[u8]>, end: Option<&[u8]>) -> () {
        self.storage.pairs(start, end);
    }
}

pub struct MapIter<'i, K, V, S>
where
    S: IterableStorage + 'i,
{
    inner: S::PairsIterator<'i>,
    phantom: PhantomData<(K, V)>,
}

impl<'i, K, V, S> Iterator for MapIter<'i, K, V, S>
where
    S: IterableStorage + 'i,
    K: Key,
    V: Storable,
{
    type Item = Result<
        (<Map<K, V> as Storable>::Key, <Map<K, V> as Storable>::Value),
        KVDecodeError<
            <Map<K, V> as Storable>::KeyDecodeError,
            <Map<K, V> as Storable>::ValueDecodeError,
        >,
    >;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| -> Self::Item {
            match (Map::<K, V>::decode_key(&k), Map::<K, V>::decode_value(&v)) {
                (Err(e), _) => Err(KVDecodeError::Key(e)),
                (_, Err(e)) => Err(KVDecodeError::Value(e)),
                (Ok(k), Ok(v)) => Ok((k, v)),
            }
        })
    }
}

pub enum KVDecodeError<K, V> {
    Key(K),
    Value(V),
}

pub trait Key {
    fn bytes(&self) -> &[u8];
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
