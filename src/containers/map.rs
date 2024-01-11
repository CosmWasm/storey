use std::{borrow::Cow, marker::PhantomData};

use crate::encoding::Encoding;

use super::{Accessor, Key, Storable};

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

    pub fn access(&self) -> MapAccess<'static, K, V, E> {
        Self::access_impl(self.prefix)
    }
}

impl<K, V, E> Storable<E> for Map<K, V, E>
where
    K: ?Sized,
    E: Encoding,
    V: Storable<E>,
{
    type AccessorT<'ns> = MapAccess<'ns, K, V, E>;

    fn access_impl<'k>(prefix: impl Into<Cow<'k, [u8]>>) -> MapAccess<'k, K, V, E> {
        MapAccess {
            namespace: prefix.into(),
            phantom: PhantomData,
        }
    }
}

pub struct MapAccess<'k, K: ?Sized, V, E> {
    namespace: Cow<'k, [u8]>,
    phantom: PhantomData<(*const K, V, E)>,
}

impl<K, V, E> MapAccess<'_, K, V, E>
where
    E: Encoding,
    K: Key + ?Sized,
    V: Storable<E>,
{
    pub fn get<'k>(&self, key: &'k K) -> V::AccessorT<'k> {
        let key = key.bytes();
        // TODO: length prefix
        let key = [&self.namespace, key].concat();
        V::access_impl(key)
    }
}

impl<K: ?Sized, V, E> Accessor for MapAccess<'_, K, V, E> {}
