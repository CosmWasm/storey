use std::marker::PhantomData;

use crate::backend::StorageBackend;
use crate::encoding::Encoding;
use crate::{DecodableWith, EncodableWith};

use super::{Accessor, Container};

pub struct NonNullItem<E, T> {
    phantom: PhantomData<(E, T)>,
}

impl<E, T> NonNullItem<E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<E, T> Container<E> for NonNullItem<E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E> + Default,
{
    type Item = T;
    type AccessorT<'ns> = ItemAccess<'ns, E, T> where E:'ns, T:'ns;

    fn init(ns: &[u8], storage: &mut impl StorageBackend) -> Result<(), E::EncodeError> {
        Self::access(ns).set(storage, &T::default())
    }

    fn access(prefix: &[u8]) -> ItemAccess<'_, E, T> {
        ItemAccess {
            namespace: prefix,
            phantom: PhantomData,
        }
    }
}

pub struct ItemAccess<'k, E, T> {
    namespace: &'k [u8],
    phantom: PhantomData<&'k (E, T)>,
}

impl<'k, E, T> ItemAccess<'k, E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub fn set(&self, storage: &mut impl StorageBackend, value: &T) -> Result<(), E::EncodeError> {
        Ok(storage.set(self.namespace, &value.encode()?))
    }

    pub fn get(&self, storage: &impl StorageBackend) -> Result<Option<T>, E::DecodeError> {
        storage
            .get(self.namespace)
            .map(|data| T::decode(&data))
            .transpose()
    }
}

impl<'ns, E: Encoding, T> Accessor<E> for ItemAccess<'ns, E, T> {
    type Item = T;
}
