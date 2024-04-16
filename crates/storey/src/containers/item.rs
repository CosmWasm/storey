use std::marker::PhantomData;

use crate::encoding::{DecodableWith, EncodableWith, Encoding};
use crate::storage::StorageBranch;
use crate::storage::{Storage, StorageMut};

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

    pub fn access<S>(&self, storage: S) -> ItemAccess<E, T, StorageBranch<S>> {
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
    pub fn set(&mut self, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;
        self.storage.set(&[], &bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mocks::backend::TestStorage;
    use mocks::encoding::TestEncoding;

    #[test]
    fn basic() {
        let mut storage = TestStorage::new();

        let item0 = Item::<u64, TestEncoding>::new(&[0]);
        item0.access(&mut storage).set(&42).unwrap();

        let item1 = Item::<u64, TestEncoding>::new(&[1]);
        let access1 = item1.access(&storage);

        assert_eq!(item0.access(&storage).get().unwrap(), Some(42));
        assert_eq!(storage.get(&[0]), Some(42u64.to_le_bytes().to_vec()));
        assert_eq!(access1.get().unwrap(), None);
        assert_eq!(storage.get(&[1]), None);
    }
}
