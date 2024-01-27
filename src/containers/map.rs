use std::{borrow::Borrow, marker::PhantomData};

use crate::{storage_branch::StorageBranch, Storage};

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
    type Value = V::Value;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, S> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
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
        let key = key.bytes();
        V::access_impl(StorageBranch::new(&self.storage, key.to_vec()))
    }
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
