pub mod column;
mod item;
mod map;

use std::marker::PhantomData;

pub use column::{Column, ColumnAccess};
pub use item::{Item, ItemAccess};
pub use map::{Map, MapAccess};

use crate::IterableStorage;

pub trait Storable {
    type AccessorT<S>;
    type Key;
    type Value;
    type ValueDecodeError;

    fn access_impl<S>(storage: S) -> Self::AccessorT<S>;

    fn decode_key(key: &[u8]) -> Result<Self::Key, KeyDecodeError>;

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError>;
}

pub struct KeyDecodeError;

pub struct StorableIter<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    inner: B::PairsIterator<'i>,
    phantom: PhantomData<S>,
}

impl<'i, S, B> Iterator for StorableIter<'i, S, B>
where
    S: Storable,
    B: IterableStorage + 'i,
{
    type Item = Result<(S::Key, S::Value), KVDecodeError<S::ValueDecodeError>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| -> Self::Item {
            match (S::decode_key(&k), S::decode_value(&v)) {
                (Err(_), _) => Err(KVDecodeError::Key),
                (_, Err(e)) => Err(KVDecodeError::Value(e)),
                (Ok(k), Ok(v)) => Ok((k, v)),
            }
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum KVDecodeError<V> {
    Key,
    Value(V),
}
