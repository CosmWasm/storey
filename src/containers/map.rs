use std::{borrow::Cow, marker::PhantomData};

use crate::{encoding::Encoding, storage_branch::StorageBranch, StorageBackend};

use super::{Key, Storable};

pub struct Map<K: ?Sized, V, E> {
    prefix: &'static [u8],
    phantom: PhantomData<(*const K, V, E)>,
}

impl<K, V, E> Map<K, V, E>
where
    K: ?Sized,
    E: Encoding,
    V: Storable<E>,
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
    ) -> MapAccess<K, V, E, StorageBranch<'s, S>> {
        Self::access_impl(storage.branch(self.prefix.to_vec()))
    }
}

impl<K, V, E> Storable<E> for Map<K, V, E>
where
    K: ?Sized,
    E: Encoding,
    V: Storable<E>,
{
    type AccessorT<S> = MapAccess<K, V, E, S>;

    fn access_impl<S>(storage: S) -> MapAccess<K, V, E, S> {
        MapAccess {
            storage,
            phantom: PhantomData,
        }
    }
}

pub struct MapAccess<K: ?Sized, V, E, S> {
    storage: S,
    phantom: PhantomData<(*const K, V, E)>,
}

impl<K, V, E, S> MapAccess<K, V, E, S>
where
    E: Encoding,
    K: Key + ?Sized,
    V: Storable<E>,
    S: StorageBackend,
{
    pub fn get<'s>(&'s self, key: &K) -> V::AccessorT<StorageBranch<'s, S>> {
        let key = key.bytes();
        V::access_impl(self.storage.branch(key.to_vec()))
    }
}
