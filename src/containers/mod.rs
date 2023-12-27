use std::{
    borrow::{Borrow, Cow},
    marker::PhantomData,
};

use crate::{
    backend::StorageBackend,
    encoding::{DecodableWith, EncodableWith, Encoding},
};

pub trait Storable<E: Encoding> {
    type AccessorT<'ns>: Accessor;

    fn access_impl<'k>(prefix: impl Into<Cow<'k, [u8]>>) -> Self::AccessorT<'k>;
}

pub struct Item<T, E> {
    prefix: &'static [u8],
    phantom: PhantomData<(T, E)>,
}

impl<T, E> Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn access(&self) -> ItemAccess<'static, E, T> {
        Self::access_impl(self.prefix)
    }
}

impl<T, E> Storable<E> for Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type AccessorT<'ns> = ItemAccess<'ns, E, T>;

    fn access_impl<'k>(prefix: impl Into<Cow<'k, [u8]>>) -> ItemAccess<'k, E, T> {
        ItemAccess {
            prefix: prefix.into(),
            phantom: PhantomData,
        }
    }
}

pub struct ItemAccess<'k, E, T> {
    prefix: Cow<'k, [u8]>,
    phantom: PhantomData<(E, T)>,
}

impl<E, T> ItemAccess<'_, E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub fn get(&self, storage: &impl StorageBackend) -> Result<Option<T>, E::DecodeError> {
        storage
            .get(self.prefix.borrow())
            .map(|bytes| T::decode(bytes.as_slice()))
            .transpose()
    }

    pub fn set(&self, storage: &impl StorageBackend, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;
        storage.set(self.prefix.borrow(), &bytes);
        Ok(())
    }
}

impl<E, T> Accessor for ItemAccess<'_, E, T> {}

pub trait Collection {
    type Item;
}

pub trait Accessor {}

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

pub trait Key {
    fn bytes(&self) -> &[u8];
}

impl Key for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
