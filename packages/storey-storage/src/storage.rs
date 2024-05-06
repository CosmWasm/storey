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
    /// The iterator should iterate over keys in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a>;

    /// Get an iterator over values.
    ///
    /// The iterator should iterate over values corresponding to keys in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a>;

    /// Get an iterator over key-value pairs.
    ///
    /// The iterator should iterate over key-value pairs in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a>;
}

impl<T: IterableStorage> IterableStorage for &T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    /// Get an iterator over keys.
    ///
    /// The iterator should iterate over keys in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    /// Get an iterator over values.
    ///
    /// The iterator should iterate over values corresponding to keys in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    /// Get an iterator over key-value pairs.
    ///
    /// The iterator should iterate over key-value pairs in lexicographical order.
    ///
    /// If `start` is `None`, the iterator should start from the first key.
    /// If `end` is `None`, the iterator should iterate until the last key.
    /// If both `start` and `end` are `None`, the iterator should iterate over all keys.
    ///
    /// The range is inclusive for `start` and exclusive for `end`.
    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

impl<T: IterableStorage> IterableStorage for &mut T {
    type KeysIterator<'a> = T::KeysIterator<'a> where Self: 'a;
    type ValuesIterator<'a> = T::ValuesIterator<'a> where Self: 'a;
    type PairsIterator<'a> = T::PairsIterator<'a> where Self: 'a;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        (**self).keys(start, end)
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        (**self).values(start, end)
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        (**self).pairs(start, end)
    }
}

/// Iteration interface for binary key-value storage in reverse order.
///
/// The iterator should iterate over key-value pairs in reverse lexicographical order of keys.
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

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a>;
    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a>;
    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a>;
}
