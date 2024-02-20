use std::marker::PhantomData;

use crate::storage_branch::StorageBranch;
use crate::{
    encoding::{DecodableWith, EncodableWith, Encoding},
    Storage, StorageMut,
};

use super::{KeyDecodeError, Storable};

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

    pub fn access<'s, S: Storage + 's>(
        &self,
        storage: &'s S,
    ) -> ItemAccess<E, T, StorageBranch<'s, S>> {
        Self::access_impl(StorageBranch::new(storage, self.prefix.to_vec()))
    }
}

impl<T, E> Storable for Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type AccessorT<S> = ItemAccess<E, T, S>;
    type Key = ();
    type Value = T;
    type ValueDecodeError = E::DecodeError;

    fn access_impl<S>(storage: S) -> ItemAccess<E, T, S> {
        ItemAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(key: &[u8]) -> Result<(), KeyDecodeError> {
        if key.is_empty() {
            Ok(())
        } else {
            Err(KeyDecodeError)
        }
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        T::decode(value)
    }
}

pub struct ItemAccess<E, T, S> {
    storage: S,
    phantom: PhantomData<(E, T)>,
}

impl<E, T, S> ItemAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: Storage,
{
    pub fn get(&self) -> Result<Option<T>, E::DecodeError> {
        self.storage
            .get(&[])
            .map(|bytes| T::decode(&bytes))
            .transpose()
    }
}

impl<E, T, S> ItemAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: StorageMut,
{
    pub fn set(&self, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;
        self.storage.set(&[], &bytes);
        Ok(())
    }
}
