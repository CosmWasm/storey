use std::marker::PhantomData;

use thiserror::Error;

use crate::encoding::{DecodableWith, EncodableWith};
use crate::{encoding::Encoding, storage_branch::StorageBranch};
use crate::{Storage, StorageMut};

use super::{KeyDecodeError, Storable};

const META_NEXT_IX: &[u8] = &[0];
const META_LEN: &[u8] = &[1];

pub struct Column<T, E> {
    prefix: &'static [u8],
    phantom: PhantomData<(T, E)>,
}

impl<T, E> Column<T, E>
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

    pub fn access<S>(&self, storage: S) -> ColumnAccess<E, T, StorageBranch<S>> {
        Self::access_impl(StorageBranch::new(storage, self.prefix.to_vec()))
    }
}

impl<T, E> Storable for Column<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type AccessorT<S> = ColumnAccess<E, T, S>;
    type Key = u32;
    type Value = T;
    type ValueDecodeError = E::DecodeError;

    fn access_impl<S>(storage: S) -> ColumnAccess<E, T, S> {
        ColumnAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(key: &[u8]) -> Result<Self::Key, KeyDecodeError> {
        let key = decode_ix(key)?;

        Ok(key)
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        T::decode(value)
    }
}

pub struct ColumnAccess<E, T, S> {
    storage: S,
    phantom: PhantomData<(E, T)>,
}

impl<E, T, S> ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: Storage,
{
    pub fn get(&self, key: u32) -> Result<Option<T>, E::DecodeError> {
        self.storage
            .get(&encode_ix(key))
            .map(|bytes| T::decode(&bytes))
            .transpose()
    }

    pub fn len(&self) -> Result<u32, LenError> {
        // TODO: bounds check + error handlinge

        self.storage
            .get_meta(META_LEN)
            .map(|bytes| {
                if bytes.len() != 4 {
                    Err(LenError::InconsistentState)
                } else {
                    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                }
            })
            .unwrap_or(Ok(0))
    }

    pub fn is_empty(&self) -> Result<bool, LenError> {
        self.len().map(|len| len == 0)
    }
}

fn decode_ix(key: &[u8]) -> Result<u32, KeyDecodeError> {
    if key.len() != 4 {
        return Err(KeyDecodeError);
    }

    let row_key = u32::from_be_bytes([key[0], key[1], key[2], key[3]]);

    Ok(row_key)
}

fn encode_ix(key: u32) -> [u8; 4] {
    key.to_be_bytes()
}

impl<E, T, S> ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: StorageMut + Storage,
{
    pub fn push(&mut self, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;

        let ix = self
            .storage
            .get_meta(META_NEXT_IX)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .unwrap_or(0);

        self.storage.set(&encode_ix(ix), &bytes);

        self.storage.set_meta(META_NEXT_IX, &(ix + 1).to_be_bytes());
        let len = self
            .storage
            .get_meta(META_LEN)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .unwrap_or(0);
        self.storage.set_meta(META_LEN, &(len + 1).to_be_bytes());

        Ok(())
    }

    pub fn update(&mut self, key: u32, value: &T) -> Result<(), UpdateError<E::EncodeError>> {
        self.storage
            .get(&encode_ix(key))
            .ok_or(UpdateError::NotFound)?;

        let bytes = value.encode()?;

        self.storage.set(&encode_ix(key), &bytes);

        Ok(())
    }

    pub fn remove(&mut self, key: u32) -> Result<(), RemoveError> {
        self.storage.remove(&encode_ix(key));

        let len = self
            .storage
            .get_meta(META_LEN)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .ok_or(RemoveError::InconsistentState)?;
        self.storage.set_meta(META_LEN, &(len - 1).to_be_bytes());

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum UpdateError<E> {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    EncodingError(E),
}

impl<E> From<E> for UpdateError<E> {
    fn from(e: E) -> Self {
        UpdateError::EncodingError(e)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum RemoveError {
    #[error("inconsistent state")]
    InconsistentState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum LenError {
    #[error("inconsistent state")]
    InconsistentState,
}
