use std::ops::Bound;

/// A read interface for binary key-value storage.
pub trait Storage {
    /// Get the value of the key.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Check if the key exists.
    fn has(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    /// Get the value of the key in the metadata namespace.
    fn get_meta(&self, _key: &[u8]) -> Option<Vec<u8>>;

    /// Check if the key exists in the metadata namespace.
    fn has_meta(&self, key: &[u8]) -> bool {
        self.get_meta(key).is_some()
    }
}

/// A trait for converting a type into one that implements [`Storage`].
///
/// This trait is meant to be implemented for a tuple of the intended type, as in
/// `impl IntoStorage<T> for (T,)`. This is to allow blanket implementations on foreign
/// types without stumbling into [E0210](https://stackoverflow.com/questions/63119000/why-am-i-required-to-cover-t-in-impl-foreigntraitlocaltype-for-t-e0210).
///  
/// Implementing this trait for foreign types allows to use those foreign types directly
/// with functions like [`Item::access`](crate::Item::access).
pub trait IntoStorage<O>: Sized {
    fn into_storage(self) -> O;
}

impl<'a, T: Storage> IntoStorage<&'a T> for (&'a T,) {
    fn into_storage(self) -> &'a T {
        self.0
    }
}

impl<'a, T: Storage> IntoStorage<&'a mut T> for (&'a mut T,) {
    fn into_storage(self) -> &'a mut T {
        self.0
    }
}

/// A write interface for binary key-value storage.
pub trait StorageMut {
    /// Set the value of the key.
    fn set(&mut self, key: &[u8], value: &[u8]);

    /// Remove the key.
    fn remove(&mut self, key: &[u8]);

    /// Set the value of the key in the metadata namespace.
    fn set_meta(&mut self, _key: &[u8], _value: &[u8]);

    /// Remove the key in the metadata namespace.
    fn remove_meta(&mut self, _key: &[u8]);
}

/// Iteration interface for binary key-value storage.
///
/// The iterator should iterate over key-value pairs in lexicographical order of keys.
pub trait IterableStorage {
    /// The type of the iterator returned by [`keys`](Self::keys).
    type KeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;

    /// The type of the iterator returned by [`values`](Self::values).
    type ValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;

    /// The type of the iterator returned by [`pairs`](Self::pairs).
    type PairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    /// Get an iterator over keys.
    ///
    /// The iterator walks keys in lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a>;

    /// Get an iterator over values.
    ///
    /// The iterator walks values corresponding to keys in lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a>;

    /// Get an iterator over key-value pairs.
    ///
    /// The iterator walks key-value pairs in lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a>;
}

impl<T: IterableStorage> IterableStorage for &T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

impl<T: IterableStorage> IterableStorage for &mut T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

/// Iteration interface for binary key-value storage in reverse order.
///
/// The iterator walks key-value pairs in reverse lexicographical order of keys.
pub trait RevIterableStorage {
    /// The type of the iterator returned by [`rev_keys`](Self::rev_keys).
    type RevKeysIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;

    /// The type of the iterator returned by [`rev_values`](Self::rev_values).
    type RevValuesIterator<'a>: Iterator<Item = Vec<u8>>
    where
        Self: 'a;

    /// The type of the iterator returned by [`rev_pairs`](Self::rev_pairs).
    type RevPairsIterator<'a>: Iterator<Item = (Vec<u8>, Vec<u8>)>
    where
        Self: 'a;

    /// Get a reverse iterator over keys.
    ///
    /// The iterator walks keys in reverse lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn rev_keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::RevKeysIterator<'a>;

    /// Get a reverse iterator over values.
    ///
    /// The iterator walks values corresponding to keys in reverse lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn rev_values<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevValuesIterator<'a>;

    /// Get a reverse iterator over key-value pairs.
    ///
    /// The iterator walks key-value pairs in reverse lexicographical order.
    ///
    /// The [`Bound`] type is used to specify either end of the range - whether it should be
    /// bounded at all, and if so, whether it should be inclusive or exclusive. See the
    /// [`Bound`] documentation for more details.
    fn rev_pairs<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevPairsIterator<'a>;
}
