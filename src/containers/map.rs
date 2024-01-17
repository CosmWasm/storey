use std::marker::PhantomData;

use crate::{storage_branch::StorageBranch, StorageBackend};

use super::{Key, Storable};

pub struct Map<K: ?Sized, V> {
    prefix: &'static [u8],
    phantom: PhantomData<(*const K, V)>,
}

impl<K, V> Map<K, V>
where
    K: ?Sized,
    V: Storable,
{
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn access<'s, S: StorageBackend + 's>(
        &self,
        storage: &'s S,
    ) -> MapAccess<K, V, StorageBranch<'s, S>> {
        Self::access_impl(storage.branch(self.prefix.to_vec()))
    }
}

impl<K, V> Storable for Map<K, V>
where
    K: ?Sized,
    V: Storable,
{
    type AccessorT<S> = MapAccess<K, V, S>;

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
    K: Key + ?Sized,
    V: Storable,
    S: StorageBackend,
{
    pub fn get<'s>(&'s self, key: &K) -> V::AccessorT<StorageBranch<'s, S>> {
        let key = key.bytes();
        V::access_impl(self.storage.branch(key.to_vec()))
    }
}
