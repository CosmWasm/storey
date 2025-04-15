use std::marker::PhantomData;

use crate::encoding::{DecodableWith, EncodableWith, Encoding};
use crate::storage::{Storage, StorageMut};

use super::common::TryGetError;
use super::{IterableStorable, Storable, Terminal};

/// A single item in the storage.
///
/// This simple container doesn't manage a namespace of keys, but simply stores a single
/// value under a single key.
///
/// # Example
/// ```
/// # use mocks::encoding::TestEncoding;
/// # use mocks::backend::TestStorage;
/// use storey::containers::{Item, router};
///
/// router! {
///     router Root {
///         0 -> item: Item<u64, TestEncoding>,
///     }
/// }
///
/// # let mut storage = TestStorage::new();
/// Root::access(&mut storage).item_mut().set(&42).unwrap();
/// assert_eq!(Root::access(&storage).item().get().unwrap(), Some(42));
/// ```
pub struct Item<T, E> {
    phantom: PhantomData<(T, E)>,
}

impl<T, E> Storable for Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type Kind = Terminal;
    type Accessor<S> = ItemAccess<E, T, S>;

    fn access_impl<S>(storage: S) -> ItemAccess<E, T, S> {
        ItemAccess {
            storage,
            phantom: PhantomData,
        }
    }
}

impl<T, E> IterableStorable for Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type Key = ();
    type KeyDecodeError = ItemKeyDecodeError;
    type Value = T;
    type ValueDecodeError = E::DecodeError;

    fn decode_key(key: &[u8]) -> Result<(), ItemKeyDecodeError> {
        if key.is_empty() {
            Ok(())
        } else {
            Err(ItemKeyDecodeError)
        }
    }

    fn decode_value(value: &[u8]) -> Result<T, E::DecodeError> {
        T::decode(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
#[error("invalid key length, expected empty key")]
pub struct ItemKeyDecodeError;

/// An accessor for an `Item`.
///
/// This type provides methods to get and set the value of the item.
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
    /// Get the value of the item.
    ///
    /// Returns `Ok(None)` if the item doesn't exist (has not been set yet).
    ///
    /// # Examples
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let storage = TestStorage::new();
    ///
    /// let access = Root::access(&storage);
    ///
    /// assert_eq!(access.item().get().unwrap(), None);
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// Root::access(&mut storage).item_mut().set(&42).unwrap();
    /// assert_eq!(Root::access(&storage).item().get().unwrap(), Some(42));
    /// ```
    pub fn get(&self) -> Result<Option<T>, E::DecodeError> {
        self.storage
            .get(&[])
            .map(|bytes| T::decode(&bytes))
            .transpose()
    }

    /// Get the value of the item.
    ///
    /// Returns [`TryGetError::Empty`] if the item doesn't exist (has not been
    /// set yet).
    ///
    /// This is similar to [`get`](Self::get), but removes one level of nesting
    /// so that you can get to your data faster, without having to unpack the
    /// [`Option`].
    ///
    /// # Examples
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// Root::access(&mut storage).item_mut().set(&42).unwrap();
    /// assert_eq!(Root::access(&storage).item().try_get().unwrap(), 42);
    /// ```
    ///
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// assert!(Root::access(&storage).item().try_get().is_err());
    /// ```
    pub fn try_get(&self) -> Result<T, TryGetError<E::DecodeError>> {
        self.get()?.ok_or_else(|| TryGetError::Empty)
    }

    /// Get the value of the item or a provided default.
    ///
    /// Returns the value of the item if it exists, otherwise returns the provided default.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// assert_eq!(Root::access(&storage).item().get_or(42).unwrap(), 42);
    /// ```
    pub fn get_or(&self, default: T) -> Result<T, E::DecodeError> {
        self.get().map(|opt| opt.unwrap_or(default))
    }
}

impl<E, T, S> ItemAccess<E, T, S>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
    S: Storage + StorageMut,
{
    /// Set the value of the item.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// Root::access(&mut storage).item_mut().set(&42).unwrap();
    /// assert_eq!(Root::access(&storage).item().get().unwrap(), Some(42));
    /// ```
    pub fn set(&mut self, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;
        self.storage.set(&[], &bytes);
        Ok(())
    }

    /// Update the value of the item.
    ///
    /// The function `f` is called with the current value of the item, if it exists.
    /// If the function returns `Some`, the item is set to the new value.
    /// If the function returns `None`, the item is removed.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// Root::access(&mut storage).item_mut().set(&42).unwrap();
    /// Root::access(&mut storage).item_mut().update(|value| value.map(|v| v + 1)).unwrap();
    /// assert_eq!(Root::access(&storage).item().get().unwrap(), Some(43));
    /// ```
    pub fn update<F>(&mut self, f: F) -> Result<(), UpdateError<E::DecodeError, E::EncodeError>>
    where
        F: FnOnce(Option<T>) -> Option<T>,
    {
        let new_value = f(self.get().map_err(UpdateError::Decode)?);
        match new_value {
            Some(value) => self.set(&value).map_err(UpdateError::Encode),
            None => {
                self.remove();
                Ok(())
            }
        }
    }

    /// Remove the value of the item.
    ///
    /// # Example
    /// ```
    /// # use mocks::encoding::TestEncoding;
    /// # use mocks::backend::TestStorage;
    /// use storey::containers::{Item, router};
    ///
    /// router! {
    ///     router Root {
    ///         0 -> item: Item<u64, TestEncoding>,
    ///     }
    /// }
    /// # let mut storage = TestStorage::new();
    ///
    /// Root::access(&mut storage).item_mut().set(&42).unwrap();
    /// Root::access(&mut storage).item_mut().remove();
    /// assert_eq!(Root::access(&storage).item().get().unwrap(), None);
    /// ```
    pub fn remove(&mut self) {
        self.storage.remove(&[]);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, thiserror::Error)]
pub enum UpdateError<D, E> {
    #[error("decode error: {0}")]
    Decode(D),
    #[error("encode error: {0}")]
    Encode(E),
}

#[cfg(test)]
mod tests {
    use crate::containers::test_utils::BranchContainer;

    use super::*;

    use mocks::backend::TestStorage;
    use mocks::encoding::TestEncoding;

    #[test]
    fn basic() {
        type Item0 = BranchContainer<0, Item<u64, TestEncoding>>;
        type Item1 = BranchContainer<1, Item<u64, TestEncoding>>;

        let mut storage = TestStorage::new();

        Item0::access(&mut storage).set(&42).unwrap();

        let access1 = Item1::access(&storage);

        assert_eq!(Item0::access(&storage).get().unwrap(), Some(42));
        assert_eq!(storage.get(&[0]), Some(42u64.to_le_bytes().to_vec()));
        assert_eq!(access1.get().unwrap(), None);
        assert_eq!(storage.get(&[1]), None);
    }

    #[test]
    fn update() {
        type MyItem = BranchContainer<0, Item<u64, TestEncoding>>;

        let mut storage = TestStorage::new();

        MyItem::access(&mut storage).set(&42).unwrap();

        MyItem::access(&mut storage)
            .update(|value| value.map(|v| v + 1))
            .unwrap();
        assert_eq!(MyItem::access(&storage).get().unwrap(), Some(43));

        MyItem::access(&mut storage).update(|_| None).unwrap();
        assert_eq!(MyItem::access(&storage).get().unwrap(), None);
    }
}
