use std::marker::PhantomData;

use storey_storage::IntoStorage;
use thiserror::Error;

use crate::encoding::Encoding;
use crate::encoding::{DecodableWith, EncodableWith};
use crate::storage::{IterableStorage, StorageBranch};
use crate::storage::{Storage, StorageMut};

use super::common::TryGetError;
use super::{BoundFor, BoundedIterableAccessor, IterableAccessor, NonTerminal, Storable};

/// The first (lowest) ID that is pushed to the column.
const FIRST_ID: u32 = 1;

/// Storage keys for metadata.
mod meta_keys {
    /// The last ID that has been pushed to the column.
    /// This does not have to be the ID of the last element as it is
    /// not reset in case the last element is removed.
    pub const META_LAST_ID: &[u8] = &[0];
    pub const META_LEN: &[u8] = &[1];
}

/// A collection of rows indexed by `u32` keys. This is somewhat similar to a traditional
/// database table with an auto-incrementing primary key. We often call column keys "IDs"
/// to differentiate them from other entities.
///
/// The ID is currently encoded as a big-endian `u32` integer.
///
/// # Example
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::Column;
///
/// let mut storage = TestStorage::new();
/// let column = Column::<u64, TestEncoding>::new(0);
/// let mut access = column.access(&mut storage);
///
/// access.push(&1337).unwrap();
/// access.push(&42).unwrap();
///
/// assert_eq!(access.get(1).unwrap(), Some(1337));
/// assert_eq!(access.get(2).unwrap(), Some(42));
/// assert_eq!(access.get(3).unwrap(), None);
/// ```
pub struct Column<T, E> {
    prefix: u8,
    phantom: PhantomData<(T, E)>,
}

impl<T, E> Column<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    /// Create a new column associated with the given storage prefix.
    ///
    /// It is the responsibility of the user to ensure the prefix is unique and does not conflict
    /// with other keys in the storage.
    ///
    /// The key provided here is used as a prefix for all keys the column itself might generate.
    pub const fn new(prefix: u8) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    /// Acquire an accessor for this column.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// // immutable accessor
    /// let storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let access = column.access(&storage);
    ///
    /// // mutable accessor
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    /// ```
    pub fn access<F, S>(&self, storage: F) -> ColumnAccess<E, T, StorageBranch<S>>
    where
        (F,): IntoStorage<S>,
    {
        let storage = (storage,).into_storage();

        Self::access_impl(StorageBranch::new(storage, vec![self.prefix]))
    }
}

impl<T, E> Storable for Column<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type Kind = NonTerminal;
    type Accessor<S> = ColumnAccess<E, T, S>;
    type Key = u32;
    type KeyDecodeError = ColumnIdDecodeError;
    type Value = T;
    type ValueDecodeError = E::DecodeError;

    fn access_impl<S>(storage: S) -> ColumnAccess<E, T, S> {
        ColumnAccess {
            storage,
            phantom: PhantomData,
        }
    }

    fn decode_key(key: &[u8]) -> Result<Self::Key, ColumnIdDecodeError> {
        let key = decode_id(key)?;

        Ok(key)
    }

    fn decode_value(value: &[u8]) -> Result<Self::Value, Self::ValueDecodeError> {
        T::decode(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid key length, expected 4 bytes of big-endian u32")]
pub struct ColumnIdDecodeError;

/// An accessor for a `Column`.
///
/// This type provides methods for interacting with the column in storage.
pub struct ColumnAccess<E, T, S> {
    storage: S,
    phantom: PhantomData<(E, T)>,
}

impl<E, T, S> IterableAccessor for ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: IterableStorage,
{
    type Storable = Column<T, E>;
    type Storage = S;

    fn storage(&self) -> &Self::Storage {
        &self.storage
    }
}

impl<E, T, S> BoundedIterableAccessor for ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: IterableStorage,
{
}

impl<T, E> BoundFor<Column<T, E>> for u32 {
    fn into_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl<E, T, S> ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: Storage,
{
    /// Get the value associated with the given ID.
    ///
    /// Returns `Ok(None)` if the entry doesn't exist (has not been set yet).
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(1337));
    /// assert_eq!(access.get(2).unwrap(), None);
    /// ```
    pub fn get(&self, id: u32) -> Result<Option<T>, E::DecodeError> {
        self.storage
            .get(&encode_id(id))
            .map(|bytes| T::decode(&bytes))
            .transpose()
    }

    /// Get the value associated with the given ID.
    ///
    /// Returns [`TryGetError::Empty`] if the entry doesn't exist (has not been
    /// set yet).
    ///
    /// This is similar to [`get`](Self::get), but removes one level of nesting
    /// so that you can get to your data faster, without having to unpack the
    /// [`Option`].
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.try_get(1).unwrap(), 1337);
    /// assert!(access.try_get(2).is_err());
    /// ```
    pub fn try_get(&self, id: u32) -> Result<T, TryGetError<E::DecodeError>> {
        self.get(id)?.ok_or(TryGetError::Empty)
    }

    /// Get the value associated with the given ID or a provided default.
    ///
    /// Returns the provided default value if the entry doesn't exist (has not been set yet).
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// assert_eq!(access.get_or(1, 42).unwrap(), 42);
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.get_or(1, 42).unwrap(), 1337);
    /// ```
    pub fn get_or(&self, id: u32, default: T) -> Result<T, E::DecodeError> {
        self.get(id).map(|value| value.unwrap_or(default))
    }

    /// Get the length of the column. This is the number of elements actually stored,
    /// taking the possibility of removed elements into account.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// assert_eq!(access.len().unwrap(), 0);
    ///
    /// access.push(&1337).unwrap();
    ///
    /// assert_eq!(access.len().unwrap(), 1);
    /// ```
    pub fn len(&self) -> Result<u32, LenError> {
        // TODO: bounds check + error handlinge

        self.storage
            .get_meta(meta_keys::META_LEN)
            .map(|bytes| {
                if bytes.len() != 4 {
                    Err(LenError::InconsistentState)
                } else {
                    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                }
            })
            .unwrap_or(Ok(0))
    }

    /// Check if the column is empty.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// assert_eq!(access.is_empty().unwrap(), true);
    ///
    /// access.push(&1337).unwrap();
    ///
    /// assert_eq!(access.is_empty().unwrap(), false);
    /// ```
    pub fn is_empty(&self) -> Result<bool, LenError> {
        self.len().map(|len| len == 0)
    }
}

fn decode_id(id: &[u8]) -> Result<u32, ColumnIdDecodeError> {
    if id.len() != 4 {
        return Err(ColumnIdDecodeError);
    }

    let row_key = u32::from_be_bytes([id[0], id[1], id[2], id[3]]);

    Ok(row_key)
}

fn encode_id(id: u32) -> [u8; 4] {
    id.to_be_bytes()
}

impl<E, T, S> ColumnAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: StorageMut + Storage,
{
    /// Append a new value to the end of the column.
    ///
    /// Returns the ID of the newly inserted value. If the column is empty, the first
    /// ID will be `1`.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// const COLUMN_KEY: u8 = 0;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(COLUMN_KEY);
    /// let mut access = column.access(&mut storage);
    ///
    /// assert_eq!(access.push(&1337).unwrap(), 1);
    /// assert_eq!(access.push(&42).unwrap(), 2);
    /// ```
    pub fn push(&mut self, value: &T) -> Result<u32, PushError<E::EncodeError>> {
        let bytes = value.encode()?;

        let id = match self
            .storage
            .get_meta(meta_keys::META_LAST_ID)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        {
            Some(last_id) => last_id.checked_add(1).ok_or(PushError::IdOverflow)?,
            None => FIRST_ID,
        };

        self.storage.set(&encode_id(id), &bytes);

        self.storage
            .set_meta(meta_keys::META_LAST_ID, &(id).to_be_bytes());
        let len = self
            .storage
            .get_meta(meta_keys::META_LEN)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .unwrap_or(0);
        self.storage
            .set_meta(meta_keys::META_LEN, &(len + 1).to_be_bytes());

        Ok(id)
    }

    /// Set the value associated with the given ID.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(1337));
    ///
    /// access.set(1, &9001).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(9001));
    /// ```
    pub fn set(&mut self, id: u32, value: &T) -> Result<(), SetError<E::EncodeError>> {
        self.storage.get(&encode_id(id)).ok_or(SetError::NotFound)?;

        let bytes = value.encode()?;

        self.storage.set(&encode_id(id), &bytes);

        Ok(())
    }

    /// Update the value associated with the given ID by applying a function to it.
    ///
    /// The provided function is called with the current value, if it exists, and should return the
    /// new value. If the function returns `None`, the value is removed.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(1337));
    ///
    /// access.update(1, |value| value.map(|v| v + 1)).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(1338));
    /// ```
    pub fn update<F>(
        &mut self,
        id: u32,
        f: F,
    ) -> Result<(), UpdateError<E::DecodeError, E::EncodeError>>
    where
        F: FnOnce(Option<T>) -> Option<T>,
    {
        let new_value = f(self.get(id).map_err(UpdateError::Decode)?);
        match new_value {
            Some(value) => self.set(id, &value).map_err(UpdateError::Set),
            None => self
                .remove(id)
                .map_err(|_| UpdateError::Set(SetError::NotFound)),
        }
    }

    /// Remove the value associated with the given ID.
    ///
    /// This operation leaves behind an empty slot in the column. The ID is not reused.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::Column;
    ///
    /// let mut storage = TestStorage::new();
    /// let column = Column::<u64, TestEncoding>::new(0);
    /// let mut access = column.access(&mut storage);
    ///
    /// access.push(&1337).unwrap();
    /// assert_eq!(access.get(1).unwrap(), Some(1337));
    ///
    /// access.remove(1).unwrap();
    /// assert_eq!(access.get(1).unwrap(), None);
    /// ```
    pub fn remove(&mut self, id: u32) -> Result<(), RemoveError> {
        self.storage.remove(&encode_id(id));

        let len = self
            .storage
            .get_meta(meta_keys::META_LEN)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .ok_or(RemoveError::InconsistentState)?;
        self.storage
            .set_meta(meta_keys::META_LEN, &(len - 1).to_be_bytes());

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum PushError<E> {
    #[error("ID overflow")]
    IdOverflow,
    #[error("{0}")]
    EncodingError(E),
}

impl<E> From<E> for PushError<E> {
    fn from(e: E) -> Self {
        PushError::EncodingError(e)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum SetError<E> {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    EncodingError(E),
}

impl<E> From<E> for SetError<E> {
    fn from(e: E) -> Self {
        SetError::EncodingError(e)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum UpdateError<D, E> {
    #[error("decode error: {0}")]
    Decode(D),
    #[error("set error: {0}")]
    Set(SetError<E>),
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

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use crate::containers::{BoundedRevIterableAccessor as _, RevIterableAccessor as _};

    use super::*;

    use mocks::backend::TestStorage;
    use mocks::encoding::TestEncoding;

    #[test]
    fn basic() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        assert_eq!(access.push(&1337).unwrap(), 1);
        assert_eq!(access.push(&42).unwrap(), 2);

        assert_eq!(access.get(1).unwrap(), Some(1337));
        assert_eq!(access.get(2).unwrap(), Some(42));
        assert_eq!(access.get(3).unwrap(), None);
        assert_eq!(access.len().unwrap(), 2);

        access.remove(1).unwrap();
        assert_eq!(access.set(1, &9001), Err(SetError::NotFound));
        access.set(2, &9001).unwrap();

        assert_eq!(access.get(1).unwrap(), None);
        assert_eq!(access.get(2).unwrap(), Some(9001));
        assert_eq!(access.len().unwrap(), 1);
    }

    #[test]
    fn remove() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        assert_eq!(access.push(&1337).unwrap(), 1);
        assert_eq!(access.push(&42).unwrap(), 2);
        assert_eq!(access.push(&17).unwrap(), 3);
        assert_eq!(access.len().unwrap(), 3);

        // remove middle
        access.remove(2).unwrap();
        assert_eq!(access.len().unwrap(), 2);

        // remove first
        access.remove(10).unwrap();
        assert_eq!(access.len().unwrap(), 1);

        // remove last
        access.remove(3).unwrap();
        assert_eq!(access.len().unwrap(), 0);

        // Above removals do not reset the auto-incrementor,
        // such that we get a fresh key for the next push.
        assert_eq!(access.push(&99).unwrap(), 4);
        assert_eq!(access.len().unwrap(), 1);
    }

    #[test]
    fn update() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        access.push(&1337).unwrap();
        access.push(&42).unwrap();
        access.push(&9001).unwrap();
        access.remove(2).unwrap();

        access.update(1, |value| value.map(|v| v + 1)).unwrap();
        assert_eq!(access.get(1).unwrap(), Some(1338));

        access.update(2, |value| value.map(|v| v + 1)).unwrap();
        assert_eq!(access.get(2).unwrap(), None);

        access.update(3, |value| value.map(|v| v + 1)).unwrap();
        assert_eq!(access.get(3).unwrap(), Some(9002));
    }

    #[test]
    fn iteration() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        access.push(&1337).unwrap();
        access.push(&42).unwrap();
        access.push(&9001).unwrap();
        access.remove(2).unwrap();

        assert_eq!(
            access.pairs().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![(1, 1337), (3, 9001)]
        );

        assert_eq!(
            access.keys().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![1, 3]
        );

        assert_eq!(
            access.values().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![1337, 9001]
        );
    }

    #[test]
    fn rev_iteration() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        access.push(&1337).unwrap();
        access.push(&42).unwrap();
        access.push(&9001).unwrap();
        access.remove(2).unwrap();

        assert_eq!(
            access.rev_pairs().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![(3, 9001), (1, 1337)]
        );

        assert_eq!(
            access.rev_keys().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![3, 1]
        );

        assert_eq!(
            access.rev_values().collect::<Result<Vec<_>, _>>().unwrap(),
            vec![9001, 1337]
        );
    }

    #[test]
    fn bounded_iteration() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        access.push(&1337).unwrap();
        access.push(&42).unwrap();
        access.push(&9001).unwrap();
        access.push(&1).unwrap();
        access.push(&2).unwrap();
        access.remove(3).unwrap();

        assert_eq!(
            access
                .bounded_pairs(Bound::Excluded(2), Bound::Included(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(4, 1), (5, 2)]
        );

        assert_eq!(
            access
                .bounded_pairs(Bound::Excluded(1), Bound::Included(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(2, 42), (4, 1), (5, 2)]
        );

        // start and end set
        assert_eq!(
            access
                .bounded_pairs(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(2, 42), (4, 1)]
        );
        assert_eq!(
            access
                .bounded_keys(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![2, 4]
        );
        assert_eq!(
            access
                .bounded_values(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![42, 1]
        );

        // end unset
        assert_eq!(
            access
                .bounded_pairs(Bound::Included(2), Bound::Unbounded)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(2, 42), (4, 1), (5, 2)]
        );
        assert_eq!(
            access
                .bounded_keys(Bound::Included(2), Bound::Unbounded)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![2, 4, 5]
        );
        assert_eq!(
            access
                .bounded_values(Bound::Included(2), Bound::Unbounded)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![42, 1, 2]
        );

        // start unset
        assert_eq!(
            access
                .bounded_pairs(Bound::Unbounded, Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(1, 1337), (2, 42), (4, 1)]
        );
        assert_eq!(
            access
                .bounded_keys(Bound::Unbounded, Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![1, 2, 4]
        );
        assert_eq!(
            access
                .bounded_values(Bound::Unbounded, Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![1337, 42, 1]
        );
    }

    #[test]
    fn bounded_rev_iteration() {
        let mut storage = TestStorage::new();

        let column = Column::<u64, TestEncoding>::new(0);
        let mut access = column.access(&mut storage);

        access.push(&1337).unwrap(); //1
        access.push(&42).unwrap(); //2
        access.push(&9001).unwrap(); //3 (removed)
        access.push(&1).unwrap(); //4
        access.push(&2).unwrap(); //5
        access.remove(3).unwrap();

        // start and end set
        assert_eq!(
            access
                .bounded_rev_pairs(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(4, 1), (2, 42)]
        );
        assert_eq!(
            access
                .bounded_rev_keys(Bound::Excluded(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![4]
        );
        assert_eq!(
            access
                .bounded_rev_keys(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![4, 2]
        );
        assert_eq!(
            access
                .bounded_rev_values(Bound::Included(2), Bound::Excluded(5))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![1, 42]
        );

        // end unset
        assert_eq!(
            access
                .bounded_rev_pairs(Bound::Included(2), Bound::Unbounded)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![(5, 2), (4, 1), (2, 42)]
        );
    }
}
